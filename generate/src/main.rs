use std::{
    borrow::Borrow,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use proc_macro2::{Span, TokenStream};
use quote::format_ident;
use syn::{Ident, Item, Type, Visibility, parse_quote, punctuated::Punctuated, token};

use generate::{Place, Places, Symbols};

fn main() {
    let src_dir = PathBuf::from(String::from(env!("OUT_DIR")));

    let dst_dir = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("Missing destination directory argument"),
    );
    dbg!(&dst_dir);

    fs::create_dir_all(&dst_dir).unwrap();

    dbg!(&src_dir);

    let mut symbols = Symbols::new();

    // Create directories, one for each module, in the destination dir.
    for_each_rs(&src_dir, &mut |path| {
        eprintln!("Processing {}", path.display());
        let relative_path = path.strip_prefix(&src_dir).unwrap();
        let mod_rs = dst_dir.join(relative_path);
        let mod_dir = mod_rs.with_extension("");
        let tests_rs = mod_dir.join("tests.rs");
        let mod_exists = mod_rs.exists();
        let tests_exists = tests_rs.exists();

        if mod_exists && tests_exists {
            eprintln!("  * Skipping");
            return Ok(());
        }
        symbols.with_module(relative_path);

        let code = fs::read_to_string(path).unwrap();
        let Transformed {
            main_file,
            tests_file,
        } = transform_tree(&symbols, syn::parse_file(&code).unwrap());

        fs::create_dir_all(&mod_dir)?;

        for (file, exists, syn_file) in [
            (tests_rs, tests_exists, tests_file),
            (mod_rs, mod_exists, main_file),
        ] {
            if exists {
                continue;
            }
            eprintln!("  * Writing {}", file.display());

            fs::write(
                &file,
                add_blank_lines(prettyplease::unparse(&syn_file)).as_bytes(),
            )?;
            Command::new("rustfmt")
                .arg(&file)
                .status()
                .unwrap_or_else(|_| panic!("Error running rustfmt {}", file.display()));
        }

        Ok(())
    })
    .unwrap();
}

fn add_blank_lines(src: String) -> String {
    let mut padded = String::with_capacity((src.len() as f64 * 1.1) as usize);

    for line in src.lines() {
        padded.push_str(line);
        padded.push('\n');

        if line.starts_with("}")
            || ((line.starts_with("pub") || line.starts_with("static")) && line.ends_with(";"))
            || line == "mod tests;"
        {
            padded.push('\n');
        }
    }
    padded
}

fn for_each_rs<F>(dir: &Path, f: &mut F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(&Path) -> Result<(), Box<dyn std::error::Error>>,
{
    for entry in dir.read_dir().unwrap() {
        let entry = entry?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "rs") {
            f(&path)?;
        } else {
            let meta = entry.metadata()?;
            if meta.is_dir() {
                for_each_rs(&path, f)?;
            }
        }
    }
    Ok(())
}

struct Transformed {
    main_file: syn::File,
    tests_file: syn::File,
}

fn transform_tree(symbols: &Symbols, syn_file: syn::File) -> Transformed {
    let cap = syn_file.items.len() * 2;

    let mut transformed = Transformed {
        main_file: syn::File {
            shebang: None,
            attrs: vec![],
            items: Vec::with_capacity(cap),
        },
        tests_file: syn::File {
            shebang: None,
            attrs: vec![],
            items: Vec::with_capacity(cap),
        },
    };
    let mut items = syn_file.items.into_iter().peekable();

    let use_stg_types = Item::Use(parse_quote! {
        use crate::stg::types::{StgInt, StgPtr, StgWord, StgWord64};
    });

    transformed.main_file.items.extend([
        Item::Use(parse_quote! {
            use crate::prelude::*;
        }),
        use_stg_types.clone(),
    ]);

    transformed.tests_file.items.extend([
        Item::Use(parse_quote! {
            use super::*;
        }),
        use_stg_types,
    ]);

    // Add original imports and exports.
    while let Some(item) = items.peek() {
        match item {
            Item::Mod(_) => transformed.main_file.items.push(items.next().unwrap()),
            Item::Use(_) => transformed.main_file.items.push(items.next().unwrap()),
            _ => break,
        }
    }

    transformed.main_file.items.push(Item::Mod(parse_quote! {
        #[cfg(test)]
        mod tests;
    }));

    for item in items {
        match item {
            Item::Const(item_const) => transform_const(symbols, item_const, &mut transformed),
            Item::Enum(mut item_enum) => {
                let places = symbols.places(&item_enum.ident);

                if places.is_empty() {
                    item_enum.vis = parse_quote! { pub(crate) };
                } else {
                    item_enum.attrs.insert(0, doc_places(places));
                }
                transformed.main_file.items.push(Item::Enum(item_enum));
            }
            Item::ForeignMod(foreign_mod) => {
                for fitem in foreign_mod.items.into_iter() {
                    match fitem {
                        syn::ForeignItem::Fn(ffn) => {
                            transform_ffn(symbols, ffn, &mut transformed);
                        }
                        syn::ForeignItem::Static(syn::ForeignItemStatic {
                            vis,
                            ident,
                            mutability,
                            ty,
                            ..
                        }) => {
                            let places = symbols.places(&ident);

                            if places.is_empty() {
                                continue;
                            }

                            let attrs = export_attrs(&ident, places);

                            let rhs: syn::Expr = match ty.as_ref() {
                                Type::Array(_) => parse_quote! { [] },
                                Type::Ptr(type_ptr) => match type_ptr.mutability {
                                    Some(_) => parse_quote! { null_mut() },
                                    None => parse_quote! { null() },
                                },
                                _ => parse_quote! { 0 },
                            };

                            transformed.main_file.items.push(Item::Static(parse_quote! {
                                #(#attrs)*
                                #vis static #mutability #ident: #ty = #rhs;
                            }));
                        }
                        fitem => panic!("Unexpected Item: {fitem:#?}"),
                    }
                }
            }
            Item::Impl(item_impl) => {
                if let Type::Path(type_path) = item_impl.self_ty.as_ref()
                    && type_path
                        .path
                        .segments
                        .first()
                        .is_some_and(|ps| ps.ident == BINDGEN_INCOMPLETE_ARRAY_FIELD)
                {
                    continue;
                }
                transformed.main_file.items.push(Item::Impl(item_impl))
            }
            Item::Struct(item_struct) => transform_struct(symbols, item_struct, &mut transformed),
            Item::Type(mut item_type) => {
                let places = symbols.places(&item_type.ident);

                if places.is_empty() {
                    item_type.vis = parse_quote! { pub(crate) };
                } else {
                    item_type.attrs.insert(0, doc_places(places));
                }
                transformed.main_file.items.push(Item::Type(item_type));
            }
            Item::Union(item_union) => transform_union(symbols, item_union, &mut transformed),
            item @ Item::Use(_) => transformed.main_file.items.push(item),
            item => panic!("Unexpected Item: {item:#?}"),
        }
    }

    transformed
}

fn transform_const(
    symbols: &Symbols,
    mut item_const: syn::ItemConst,
    transformed: &mut Transformed,
) {
    let ident = item_const.ident.clone();

    if ident == "_" {
        transformed.tests_file.items.push(Item::Const(item_const));
        return;
    } else {
        let s = ident.to_string();

        if s.starts_with("FMT_") || s.contains("_FMT_") {
            eprintln!("  * Ignoring formatting const: {ident}");
            return;
        }
    }

    let places = symbols.places(&ident);

    if places.is_empty() {
        item_const.vis = parse_quote! { pub(crate) };
    } else {
        item_const.attrs.insert(0, doc_places(places));
    };
    transformed.main_file.items.push(Item::Const(item_const));

    let test_eq = format_ident!("sys_eq_{}", ident);

    transformed.tests_file.items.push(Item::Fn(parse_quote! {
        #[cfg(feature = "sys")]
        #[test]
        fn #test_eq() {
            assert_eq!(sys::#ident, #ident);
        }
    }));
}

fn transform_ffn(symbols: &Symbols, ffn: syn::ForeignItemFn, transformed: &mut Transformed) {
    let syn::ForeignItemFn {
        sig:
            syn::Signature {
                ident,
                generics,
                inputs,
                variadic,
                output,
                ..
            },
        ..
    } = ffn;

    assert!(generics.gt_token.is_none() && generics.where_clause.is_none());

    let places = symbols.places(&ident);

    if places.is_empty() {
        return;
    }

    let Transformed {
        main_file,
        tests_file,
    } = transformed;

    // TODO: There are variadic functions in rts::messages.
    if variadic.is_some() {
        eprintln!("  * Ignoring variadic function: {ident}");
        return;
    }

    #[expect(clippy::type_complexity)]
    let (inputs_owned, args_from_sys, args_into, args_from_owned, bindings): (
        Punctuated<_, token::Comma>,
        Vec<syn::Expr>,
        Vec<syn::Expr>,
        Vec<syn::Pat>,
        Vec<TokenStream>,
    ) = inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident @ syn::PatIdent { .. }) = pat_type.pat.as_ref() {
                    let param_ident = pat_ident.ident.clone();
                    let pat_ty = pat_type.ty.as_ref();

                    let (ty_owned, arg_from_sys, arg_into, arg_from_owned) = match pat_ty {
                        ty @ Type::Path(type_path) => {
                            let is_primitive = is_primitive_type_path(symbols, type_path);
                            (
                                ty.clone(),
                                if is_primitive {
                                    parse_quote! { #param_ident }
                                } else {
                                    parse_quote! { transmute(#param_ident) }
                                },
                                if is_primitive {
                                    parse_quote! { #param_ident }
                                } else {
                                    expr_into(&param_ident)
                                },
                                syn::Pat::Ident(new_pat_ident(&param_ident)),
                            )
                        }
                        Type::Ptr(type_ptr) => {
                            let (ty, expr, pat) =
                                ptr_to_ty_expr_pat(symbols, &param_ident, type_ptr);
                            let arg_from_sys = if is_primitive_type(symbols, type_ptr.elem.as_ref())
                            {
                                parse_quote! { #param_ident }
                            } else {
                                let sys_pat_ty = prefix_with_sys(pat_ty);
                                parse_quote! { #param_ident as #sys_pat_ty }
                            };
                            (ty, arg_from_sys, expr, pat)
                        }
                        ty => panic!("Unexpected type in {arg:?}: {ty:?}"),
                    };

                    let binding: TokenStream = {
                        let binding_rhs = match pat_ty {
                            Type::Ptr(type_ptr) => {
                                if type_ptr.mutability.is_some() {
                                    "null_mut()"
                                } else {
                                    "null()"
                                }
                            }
                            _ => {
                                if is_primitive_type(symbols, pat_ty) {
                                    "Default::default()"
                                } else {
                                    "todo!()"
                                }
                            }
                        };
                        parse_token_stream(format!("let {} = {};", &param_ident, binding_rhs))
                    };

                    (
                        syn::FnArg::Typed(syn::PatType {
                            attrs: pat_type.attrs.clone(),
                            pat: pat_type.pat.clone(),
                            colon_token: pat_type.colon_token,
                            ty: Box::new(ty_owned),
                        }),
                        arg_from_sys,
                        arg_into,
                        arg_from_owned,
                        binding,
                    )
                } else {
                    panic!("Expected only syn::Pat::Ident: {arg:#?}");
                }
            }
            syn::FnArg::Receiver(_) => {
                panic!("Unexpected FnArg::Recever: {arg:#?}")
            }
        })
        .collect();

    let attrs = export_attrs(&ident, places);

    let (call, expected_call): (syn::Expr, syn::Expr) = match &output {
        syn::ReturnType::Type(_, ty) if !is_primitive_type(symbols, ty.as_ref()) => (
            parse_quote! { transmute(sys::#ident(#(#args_from_sys),*)) },
            parse_quote! { transmute(sys::#ident(#(#args_into),*)) },
        ),
        _ => (
            parse_quote! { sys::#ident(#(#args_from_sys),*) },
            parse_quote! { sys::#ident(#(#args_into),*) },
        ),
    };

    // Mark all functions as unsafe until the code can be audited.
    main_file.items.push(Item::Fn(parse_quote! {
        #(#attrs)*
        #[instrument]
        pub unsafe extern "C" fn #ident(#inputs) #output {
            unsafe { #call }
        }
    }));

    let fn_ident = format_ident!("equivalent_{}", ident);

    if let syn::ReturnType::Type(_, ty) = output
        && !is_indirect(&ty)
        && !inputs
            .iter()
            .any(|arg| matches!(arg, syn::FnArg::Typed(syn::PatType { ty, .. }) if is_indirect(ty)))
    {
        tests_file.items.push(Item::Fn(parse_quote! {
            #[cfg(feature = "sys")]
            #[quickcheck]
            #[ignore]
            fn #fn_ident(#inputs_owned) -> bool {
                let expected = unsafe { #expected_call };
                let actual = unsafe { #ident(#(#args_from_owned),*) };
                actual == expected
            }
        }));
    } else {
        tests_file.items.push(Item::Fn(parse_quote! {
            #[cfg(feature = "sys")]
            #[test]
            #[ignore]
            fn #fn_ident() {
                todo!()
            }
        }));
    }
    let fn_ident = format_ident!("test_{}", ident);

    tests_file.items.push(Item::Fn(parse_quote! {
        #[test]
        #[ignore]
        fn #fn_ident() {
            #(#bindings)*
            unsafe { #ident(#(#args_from_owned),*) };
            todo!("assert")
        }
    }));
}

fn export_attrs(ident: &Ident, places: Places) -> Vec<syn::Attribute> {
    let export_name = parse_token_stream(format!("\"rust_{ident}\""));

    let mut attrs = Vec::with_capacity(3);

    if !places.is_empty() {
        attrs.push(attr_places(places));
    }

    attrs.extend([
        parse_quote! { #[cfg_attr(feature = "sys", unsafe(export_name = #export_name))] },
        parse_quote! { #[cfg_attr(not(feature = "sys"), unsafe(no_mangle))] },
    ]);

    attrs
}

fn is_indirect(ty: &Type) -> bool {
    match ty {
        Type::Ptr(_) | Type::Reference(_) => true,
        Type::Path(syn::TypePath { path, .. }) => path
            .segments
            .last()
            .is_some_and(|p| p.ident.to_string().ends_with("Ptr")),
        _ => false,
    }
}

fn is_primitive_type<T: Borrow<Type>>(symbols: &Symbols, ty: T) -> bool {
    match ty.borrow() {
        Type::Array(type_array) => is_primitive_type(symbols, type_array.elem.borrow()),
        Type::BareFn(type_bare_fn) => {
            type_bare_fn
                .inputs
                .iter()
                .any(|arg| is_primitive_type(symbols, arg.ty.borrow()))
                || match &type_bare_fn.output {
                    syn::ReturnType::Default => true,
                    syn::ReturnType::Type(_, rty) => is_primitive_type(symbols, rty.as_ref()),
                }
        }
        Type::Never(_) => true,
        Type::Path(type_path) => is_primitive_type_path(symbols, type_path),
        Type::Ptr(type_ptr) => is_primitive_type(symbols, type_ptr.elem.as_ref()),
        ty => panic!("Unexpected type: {ty:?}"),
    }
}

fn is_primitive_type_path<T: Borrow<syn::TypePath>>(symbols: &Symbols, type_path: T) -> bool {
    let type_path = type_path.borrow();
    type_path.path.leading_colon.is_some() || symbols.is_primitive_type(type_path)
}

fn expr_into(ident: &Ident) -> syn::Expr {
    syn::Expr::MethodCall(parse_quote! { #ident.into() })
}

fn new_pat_ident(ident: &Ident) -> syn::PatIdent {
    syn::PatIdent {
        attrs: vec![],
        by_ref: None,
        mutability: None,
        ident: ident.clone(),
        subpat: None,
    }
}

fn ptr_to_ty_expr_pat(
    symbols: &Symbols,
    ident: &Ident,
    type_ptr: &syn::TypePtr,
) -> (syn::Type, syn::Expr, syn::Pat) {
    let (ty, expr, pat) = match type_ptr.elem.as_ref() {
        ty @ Type::Path(type_path) => (
            ty.clone(),
            if is_primitive_type_path(symbols, type_path) {
                parse_quote! { #ident }
            } else {
                expr_into(ident)
            },
            syn::Pat::Ident(new_pat_ident(ident)),
        ),
        Type::Ptr(type_ptr) => ptr_to_ty_expr_pat(symbols, ident, type_ptr),
        _ => panic!("Unexpected type for {ident}: {type_ptr:?}"),
    };

    (
        ty,
        syn::Expr::Reference(syn::ExprReference {
            attrs: vec![],
            and_token: token::And(Span::mixed_site()),
            mutability: type_ptr.mutability,
            expr: Box::new(expr),
        }),
        pat,
    )
}

static BINDGEN_INCOMPLETE_ARRAY_FIELD: &str = "__IncompleteArrayField";

fn transform_struct(
    symbols: &Symbols,
    mut item_struct: syn::ItemStruct,
    Transformed {
        main_file,
        tests_file,
    }: &mut Transformed,
) {
    if item_struct.ident == BINDGEN_INCOMPLETE_ARRAY_FIELD {
        return;
    }
    let ident = item_struct.ident.clone();

    let places = symbols.places(&ident);

    if places.is_empty() || ident.to_string().ends_with("_") {
        item_struct
            .attrs
            .push(parse_quote! { #[doc = " cbindgen:no-export"] });

        if let syn::Fields::Named(syn::FieldsNamed { named, .. }) = &mut item_struct.fields {
            for f in named {
                f.vis = Visibility::Inherited;
            }
        }
    } else {
        item_struct.attrs.insert(0, doc_places(places));
    }

    let arbitrary_fields = if let syn::Fields::Named(syn::FieldsNamed { named, .. }) =
        &item_struct.fields
        && named.iter().all(|f| {
            symbols.is_simple_type(&f.ty) && f.ident.as_ref().is_some_and(|i| i != "_unused")
        }) {
        Some(named.clone())
    } else {
        None
    };

    main_file
        .items
        .extend([Item::Struct(item_struct), Item::Impl(impl_from(&ident))]);

    if let Some(fields) = arbitrary_fields {
        main_file
            .items
            .push(Item::Impl(impl_arbitrary(&ident, &fields)));
    }

    tests_file.items.push(Item::Fn(fn_test_size_of(&ident)));
}

fn transform_union(
    symbols: &Symbols,
    mut item_union: syn::ItemUnion,
    Transformed {
        main_file,
        tests_file,
    }: &mut Transformed,
) {
    let ident = item_union.ident.clone();

    let places = symbols.places(&ident);

    if places.is_empty() {
        item_union.vis = parse_quote! { pub(crate) };
    } else {
        item_union.attrs.insert(0, attr_places(places));
    }

    if places.is_empty() || places == Place::Testsuite {
        for f in item_union.fields.named.iter_mut() {
            f.vis = Visibility::Inherited;
        }
    }

    // Remove ManuallyDrop for primitive/pointer types.
    for f in item_union.fields.named.iter_mut() {
        if let Type::Path(type_path) = &f.ty
            && let Some(ps) = type_path.path.segments.last()
            && ps.ident == "ManuallyDrop"
            && let syn::PathArguments::AngleBracketed(angle_args) = &ps.arguments
            && angle_args.args.len() == 1
            && let Some(syn::GenericArgument::Type(param_ty)) = angle_args.args.first()
            && (matches!(param_ty, Type::Path(tp) if symbols.is_primitive_type(tp))
                || symbols.is_pointer_type(param_ty))
        {
            f.ty = param_ty.clone();
        }
    }

    main_file
        .items
        .extend([Item::Union(item_union), Item::Impl(impl_from(&ident))]);

    tests_file.items.push(Item::Fn(fn_test_size_of(&ident)));
}

fn parse_token_stream<S>(s: S) -> TokenStream
where
    S: AsRef<str> + std::fmt::Display,
{
    s.as_ref()
        .parse::<TokenStream>()
        .unwrap_or_else(|e| panic!("Unable to parse TokenStream: {s}: {e}"))
}

fn impl_from(ident: &Ident) -> syn::ItemImpl {
    parse_quote! {
        #[cfg(feature = "sys")]
        impl From<#ident> for sys::#ident {
            fn from(x: #ident) -> Self {
                unsafe { transmute(x) }
            }
        }
    }
}

fn impl_arbitrary<'a, I: IntoIterator<Item = &'a syn::Field>>(
    ident: &Ident,
    fields: I,
) -> syn::ItemImpl {
    let arbitrary_fields: Vec<_> = fields
        .into_iter()
        .map(|field| {
            parse_token_stream(format!(
                "{}: Arbitrary::arbitrary(g)",
                field.ident.clone().unwrap()
            ))
        })
        .collect();

    parse_quote! {
        #[cfg(test)]
        impl Arbitrary for #ident {
            fn arbitrary(g: &mut Gen) -> Self {
                #ident {
                    #(#arbitrary_fields),*
                }
            }
        }
    }
}

fn fn_test_size_of(ident: &Ident) -> syn::ItemFn {
    let test_size_of = format_ident!("sys_size_{}", ident);

    parse_quote! {
        #[cfg(feature = "sys")]
        #[test]
        fn #test_size_of() {
            assert_eq!(size_of::<sys::#ident>(), size_of::<#ident>())
        }
    }
}

fn prefix_with_sys<T: Borrow<Type>>(ty: T) -> Type {
    match ty.borrow() {
        Type::Array(type_array) => {
            let mut array = type_array.clone();
            array.elem = Box::new(prefix_with_sys(type_array.elem.as_ref()));
            Type::Array(array)
        }
        Type::Path(type_path) => parse_quote! { sys::#type_path }, // is_primitive_type_path(symbols, type_path),
        Type::Ptr(type_ptr) => {
            let mut ptr = type_ptr.clone();
            ptr.elem = Box::new(prefix_with_sys(type_ptr.elem.as_ref()));
            Type::Ptr(ptr)
        }
        ty => panic!("Unexpected type: {ty:?}"),
    }
}

fn attr_places(places: Places) -> syn::Attribute {
    if places == generate::Place::Testsuite {
        parse_quote! { #[cfg(feature = "ghc_testsuite")] }
    } else {
        doc_places(places)
    }
}

fn doc_places(places: Places) -> syn::Attribute {
    let mut s = " - GHC_PLACES: {".to_string();
    let mut is_first = true;

    for p in places {
        if is_first {
            is_first = false;
        } else {
            s.push_str(", ");
        }
        s.push_str(p.to_str());
    }
    s.push('}');
    parse_quote! { #[doc = #s] }
}
