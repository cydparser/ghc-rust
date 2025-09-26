use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use proc_macro2 as proc2;
use proc_macro2::{Span, TokenStream};
use quote::format_ident;
use syn::{Ident, Item, Type, Visibility, parse_quote, punctuated::Punctuated, token};

use generate::{Place, Places, Symbols, prefix_with_sys};

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
    let mut imports = true;

    for line in src.lines() {
        if imports && !line.starts_with("use") {
            imports = false;
            padded.push('\n')
        }
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

                item_enum.attrs.push(parse_quote! { #[derive(Copy)] });

                let impl_arb = if symbols.is_simple(&item_enum.ident) {
                    Some(Item::Impl(impl_arbitrary_enum(
                        &item_enum.ident,
                        &item_enum.variants,
                    )))
                } else {
                    None
                };

                transformed.main_file.items.push(Item::Enum(item_enum));

                if let Some(impl_arb) = impl_arb {
                    transformed.main_file.items.push(impl_arb);
                }
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
                                Type::Path(type_path) => {
                                    if let Some(ps) = type_path.path.segments.last()
                                        && ps.ident == "bool"
                                    {
                                        parse_quote! { false }
                                    } else {
                                        parse_quote! { 0 }
                                    }
                                }
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

        if s.starts_with("FMT_") || s.contains("_FMT_") || s.ends_with("_FMT") {
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
    let sig @ syn::Signature {
        ident,
        generics,
        inputs,
        variadic,
        output,
        ..
    } = &ffn.sig;

    assert!(generics.gt_token.is_none() && generics.where_clause.is_none());

    let places = symbols.places(ident);

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

    let mut inputs_owned: Punctuated<syn::FnArg, token::Comma> = Punctuated::new();
    let mut args_from_sys: Vec<syn::Expr> = vec![];
    let mut args_into: Vec<syn::Expr> = vec![];
    let mut args_from_owned: Vec<syn::Pat> = vec![];
    let mut bindings: Vec<TokenStream> = vec![];

    for arg in inputs.iter() {
        match arg {
            syn::FnArg::Receiver(_) => {
                panic!("Unexpected FnArg::Recever: {arg:#?}")
            }
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident @ syn::PatIdent { .. }) = pat_type.pat.as_ref() {
                    let param_ident = pat_ident.ident.clone();
                    let pat_ty = pat_type.ty.as_ref();

                    let (ty_owned, arg_from_sys, arg_into, arg_from_owned) = match pat_ty {
                        ty @ Type::Path(type_path) => {
                            let is_std = symbols.is_std_type_path(type_path);
                            (
                                ty.clone(),
                                if is_std {
                                    parse_quote! { #param_ident }
                                } else if symbols.is_pointer_type(ty) {
                                    let sys_pat_ty = prefix_with_sys(ty);
                                    parse_quote! { #param_ident as #sys_pat_ty }
                                } else {
                                    parse_quote! { transmute(#param_ident) }
                                },
                                if is_std {
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
                            let arg_from_sys = if symbols.is_std_type(type_ptr.elem.as_ref()) {
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
                                if symbols.is_simple_type(pat_ty) {
                                    "Arbitrary::arbitrary()"
                                } else {
                                    "todo!()"
                                }
                            }
                        };
                        parse_token_stream(format!("let {} = {};", &param_ident, binding_rhs))
                    };

                    inputs_owned.push(syn::FnArg::Typed(syn::PatType {
                        attrs: pat_type.attrs.clone(),
                        pat: pat_type.pat.clone(),
                        colon_token: pat_type.colon_token,
                        ty: Box::new(ty_owned),
                    }));
                    args_from_sys.push(arg_from_sys);
                    args_into.push(arg_into);
                    args_from_owned.push(arg_from_owned);
                    bindings.push(binding);
                } else {
                    panic!("Expected only syn::Pat::Ident: {arg:#?}");
                }
            }
        }
    }

    let attrs = export_attrs(ident, places);

    let call: syn::Expr = match &output {
        syn::ReturnType::Type(_, ret_ty) if !symbols.is_std_type(ret_ty.as_ref()) => {
            if matches!(ret_ty.as_ref(), Type::Ptr(_)) || symbols.is_pointer_type(ret_ty.as_ref()) {
                parse_quote! { sys::#ident(#(#args_from_sys),*) as #ret_ty }
            } else {
                parse_quote! { transmute(sys::#ident(#(#args_from_sys),*)) }
            }
        }
        _ => parse_quote! { sys::#ident(#(#args_from_sys),*) },
    };

    // Mark all functions as unsafe until the code can be audited.
    main_file.items.push(Item::Fn(parse_quote! {
        #(#attrs)*
        #[instrument]
        pub unsafe extern "C" fn #ident(#inputs) #output {
            unsafe { #call }
        }
    }));

    if let Some(tests) = generate::generate_tests(symbols, sig) {
        for test in tests {
            tests_file.items.push(syn::Item::Fn(test));
        }
    }
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
            if symbols.is_primitive_type_path(type_path) {
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
            .insert(0, parse_quote! { #[doc = " cbindgen:no-export"] });

        if let syn::Fields::Named(syn::FieldsNamed { named, .. }) = &mut item_struct.fields {
            for f in named {
                f.vis = Visibility::Inherited;
            }
        }
    } else {
        item_struct.attrs.insert(0, doc_places(places));
    }

    let impl_arb = if symbols.is_simple(&item_struct.ident) {
        match &item_struct.fields {
            syn::Fields::Named(fs)
                if fs.named.len() == 1
                    && fs
                        .named
                        .first()
                        .is_some_and(|f| f.ident.as_ref().is_some_and(|i| i == "_unused")) =>
            {
                None
            } // Opaque type
            fields => {
                item_struct
                    .attrs
                    .push(parse_quote! { #[cfg_attr(test, derive(Clone))] });
                Some(Item::Impl(impl_arbitrary_struct(&ident, fields)))
            }
        }
    } else {
        None
    };

    main_file
        .items
        .extend([Item::Struct(item_struct), Item::Impl(impl_from(&ident))]);

    if let Some(impl_arb) = impl_arb {
        main_file.items.push(impl_arb);
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
            && (symbols.is_primitive_type(param_ty) || symbols.is_pointer_type(param_ty))
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

fn impl_arbitrary_struct(ident: &Ident, fields: &syn::Fields) -> syn::ItemImpl {
    let arbitrary_fields = arbitrary_data_constructor(ident, fields);

    parse_quote! {
        #[cfg(test)]
        impl Arbitrary for #ident {
            fn arbitrary(g: &mut Gen) -> Self {
                #arbitrary_fields
            }
        }
    }
}

fn impl_arbitrary_enum(
    ident: &Ident,
    variants: &Punctuated<syn::Variant, token::Comma>,
) -> syn::ItemImpl {
    let variant_count = variants.len();

    let arbitrary_variants: Vec<syn::Arm> = variants
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            let arbitrary_variant = arbitrary_data_constructor(&v.ident, &v.fields);

            let pat = if i < variant_count - 1 {
                syn::Pat::Lit(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::Lit::Int(proc2::Literal::usize_unsuffixed(i).into()),
                })
            } else {
                let i = syn::Lit::Int(proc2::Literal::usize_unsuffixed(i).into());
                parse_quote! { #i.. }
            };

            parse_quote! {
                #pat => #arbitrary_variant
            }
        })
        .collect();

    let variant_count = syn::Lit::Int(proc2::Literal::usize_unsuffixed(variant_count).into());

    parse_quote! {
        #[cfg(test)]
        impl Arbitrary for #ident {
            fn arbitrary(g: &mut Gen) -> Self {
                use #ident::*;
                match usize::arbitrary(g) % #variant_count {
                    #(#arbitrary_variants),*
                }
            }
        }
    }
}

fn arbitrary_data_constructor(ident: &Ident, fields: &syn::Fields) -> syn::Expr {
    match fields {
        syn::Fields::Named(fields_named) => {
            let arbitrary_fields: Vec<_> = fields_named
                .named
                .iter()
                .map(|f| {
                    parse_token_stream(format!(
                        "{}: Arbitrary::arbitrary(g)",
                        f.ident.clone().unwrap()
                    ))
                })
                .collect();

            parse_quote! {
                #ident {
                    #(#arbitrary_fields),*
                }
            }
        }
        syn::Fields::Unnamed(fields_unnamed) => {
            let arbitrary_fields: Vec<syn::Expr> = std::iter::repeat_n(
                parse_quote! { Arbitrary::arbitrary(g) },
                fields_unnamed.unnamed.len(),
            )
            .collect();

            parse_quote! {
                #ident(#(#arbitrary_fields),*)
            }
        }
        syn::Fields::Unit => parse_quote!(#ident),
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
