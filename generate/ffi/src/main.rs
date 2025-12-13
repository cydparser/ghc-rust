use std::{
    fs, iter,
    path::{Path, PathBuf},
    process::Command,
};

use proc_macro2 as proc2;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, Item, Type, Visibility, parse_quote, punctuated::Punctuated, token};

use generate_consumers::{Consumer, Consumers};
use generate_ffi::{Symbols, enums, fields, prefix_with_sys};

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

    transformed.main_file.items.push(Item::Use(parse_quote! {
        use crate::prelude::*;
    }));

    transformed
        .tests_file
        .items
        .extend([Item::Use(parse_quote! {
            use super::*;
        })]);

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
            Item::Enum(item_enum) => transform_enum(symbols, item_enum, &mut transformed),
            Item::ForeignMod(foreign_mod) => {
                for fitem in foreign_mod.items.into_iter() {
                    match fitem {
                        syn::ForeignItem::Fn(ffn) => {
                            transform_ffn(symbols, ffn, &mut transformed);
                        }
                        syn::ForeignItem::Static(item_static) => {
                            transform_static(symbols, item_static, &mut transformed)
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
            Item::Type(item_type) => transform_type(symbols, item_type, &mut transformed),
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
        return;
    } else {
        let s = ident.to_string();

        if s.starts_with("FMT_") || s.contains("_FMT_") || s.ends_with("_FMT") {
            eprintln!("  * Ignoring formatting const: {ident}");
            return;
        }
    }

    let consumers = symbols.consumers(&ident);

    if consumers.is_empty() {
        item_const.vis = parse_quote! { pub(crate) };
    } else {
        item_const.attrs.insert(0, attr_ffi(consumers));

        let test_eq = format_ident!("sys_{}_eq", ident);

        transformed.tests_file.items.extend([
            Item::Fn(parse_quote! {
                #[cfg(feature = "sys")]
                #[test]
                fn #test_eq() {
                    assert_eq!(#ident, sys::#ident);
                }
            }),
            Item::Fn(fn_test_layout_of_val(&ident, true, true, None)),
        ]);
    };
    transformed.main_file.items.push(Item::Const(item_const));
}

fn transform_enum(symbols: &Symbols, mut item_enum: syn::ItemEnum, transformed: &mut Transformed) {
    let ident = item_enum.ident.clone();
    let variants = &item_enum.variants;

    let consumers = symbols.consumers(&ident);

    if consumers.is_empty() {
        item_enum.vis = parse_quote! { pub(crate) };
    } else {
        item_enum.attrs.insert(0, attr_ffi(consumers));

        transformed.tests_file.items.extend([
            Item::Fn(fn_test_layout(symbols, &ident)),
            Item::Fn(enums::test_discriminants(&ident, variants)),
        ]);
    }

    item_enum.attrs.push(parse_quote! { #[derive(Copy)] });

    let impl_froms = enums::impl_froms(&ident, variants)
        .into_iter()
        .map(Item::Impl);

    let impl_try_from = Item::Impl(enums::impl_try_from_u32(&ident, variants));

    let impl_arb = if symbols.is_simple(&ident) {
        Some(Item::Impl(impl_arbitrary_enum(&ident, variants)))
    } else {
        None
    };

    transformed.main_file.items.extend(
        [Item::Enum(item_enum)]
            .into_iter()
            .chain(impl_froms)
            .chain(iter::once(impl_try_from)),
    );

    if let Some(impl_arb) = impl_arb {
        transformed.main_file.items.push(impl_arb);
    }
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

    let consumers = symbols.consumers(ident);

    if consumers.is_empty() {
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
                                } else if symbols.is_pointer_type(ty) && !symbols.is_option_type(ty)
                                {
                                    parse_quote! { #param_ident.cast() }
                                } else {
                                    parse_quote! { transmute(#param_ident) }
                                },
                                if is_std {
                                    parse_quote! { #param_ident }
                                } else {
                                    parse_quote! { transmute(#param_ident) }
                                },
                                syn::Pat::Ident(new_pat_ident(&param_ident)),
                            )
                        }
                        Type::Ptr(type_ptr) => {
                            let (ty, expr, pat) = ptr_to_ty_expr_pat(&param_ident, type_ptr);
                            let arg_from_sys = if symbols.is_std_type(type_ptr.elem.as_ref()) {
                                parse_quote! { #param_ident }
                            } else {
                                let sys_pat_ty = prefix_with_sys(symbols, pat_ty);
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

    let attrs = export_attrs(consumers);

    let ret_ty = match output {
        syn::ReturnType::Type(_, ret_ty) => Some(ret_ty.as_ref()),
        _ => None,
    };

    let call: syn::Expr = {
        let mut call = quote! { #ident(#(#args_from_sys),*) };

        match ret_ty {
            Some(ret_ty) if !symbols.is_std_type(ret_ty) => {
                if symbols.is_pointer_type(ret_ty) {
                    call = quote! { #call.cast() };
                } else {
                    call = quote! { transmute(#call) };
                }
            }
            _ => (),
        };
        parse_quote! {
            sys! { #call }
        }
    };

    let (instrument, before_exit) = match ret_ty {
        Some(Type::Never(_)) => {
            let msg = ident.to_string();
            (None, Some(quote! { before_exit(#msg); }))
        }
        _ => (Some(quote! { #[instrument] }), None),
    };

    // Mark all functions as unsafe until the code can be audited.
    main_file.items.push(Item::Fn(parse_quote! {
        #(#attrs)*
        #instrument
        pub unsafe extern "C" fn #ident(#inputs) #output {
            #before_exit
            #call
        }
    }));

    if let Some(tests) = generate_ffi::generate_tests(symbols, sig) {
        for test in tests {
            tests_file.items.push(syn::Item::Fn(test));
        }
    }
}

fn transform_static(
    symbols: &Symbols,
    item_static: syn::ForeignItemStatic,
    transformed: &mut Transformed,
) {
    let syn::ForeignItemStatic {
        vis,
        ident,
        mutability,
        ty,
        ..
    } = item_static;
    let consumers = symbols.consumers(&ident);

    if consumers.is_empty() {
        return;
    }

    let attrs = export_attrs(consumers);

    let rhs: syn::Expr = match ty.as_ref() {
        Type::Array(_) => parse_quote! { [0; _] },
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

    let mutable = mutability != syn::StaticMutability::None;

    transformed
        .tests_file
        .items
        .push(Item::Fn(fn_test_layout_of_val(
            &ident,
            !mutable,
            false,
            if mutable {
                Some(parse_quote!(#[expect(static_mut_refs)]))
            } else {
                None
            },
        )));
}

fn export_attrs(consumers: Consumers) -> Vec<syn::Attribute> {
    let mut attrs = Vec::with_capacity(3);

    if !consumers.is_empty() {
        attrs.push(attr_ffi(consumers));
    }

    attrs.extend([parse_quote! { #[unsafe(no_mangle)] }]);

    attrs
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

fn ptr_to_ty_expr_pat(ident: &Ident, type_ptr: &syn::TypePtr) -> (syn::Type, syn::Expr, syn::Pat) {
    let (ty, expr, pat) = match type_ptr.elem.as_ref() {
        ty @ Type::Path(_) => (
            ty.clone(),
            parse_quote! { #ident },
            syn::Pat::Ident(new_pat_ident(ident)),
        ),
        Type::Ptr(type_ptr) => ptr_to_ty_expr_pat(ident, type_ptr),
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

    let consumers = symbols.consumers(&ident);

    if consumers.is_empty() || ident.to_string().ends_with("_") {
        item_struct
            .attrs
            .insert(0, parse_quote! { #[doc = " cbindgen:no-export"] });

        if let syn::Fields::Named(syn::FieldsNamed { named, .. }) = &mut item_struct.fields {
            for f in named {
                f.vis = Visibility::Inherited;
            }
        }
    } else {
        item_struct.attrs.insert(0, attr_ffi(consumers));

        tests_file.items.push(Item::Fn(fields::test_layout(
            symbols,
            &ident,
            &item_struct.fields,
        )));
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

    main_file.items.push(Item::Struct(item_struct));

    if let Some(impl_arb) = impl_arb {
        main_file.items.push(impl_arb);
    }
}

fn transform_type(symbols: &Symbols, mut item_type: syn::ItemType, transformed: &mut Transformed) {
    let consumers = symbols.consumers(&item_type.ident);

    if consumers.is_empty() {
        item_type.vis = parse_quote! { pub(crate) };
    } else {
        item_type.attrs.insert(0, attr_ffi(consumers));

        transformed
            .tests_file
            .items
            .push(Item::Fn(fn_test_layout(symbols, &item_type.ident)));
    }
    transformed.main_file.items.push(Item::Type(item_type));
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

    let consumers = symbols.consumers(&ident);

    if consumers.is_empty() {
        item_union.vis = parse_quote! { pub(crate) };
        item_union
            .attrs
            .insert(0, parse_quote! { #[doc = " cbindgen:no-export"] });
    } else {
        item_union.attrs.insert(0, attr_ffi(consumers));
    }

    if consumers.is_empty() || consumers == Consumer::Testsuite {
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

    main_file.items.push(Item::Union(item_union));

    tests_file
        .items
        .push(Item::Fn(fn_test_layout(symbols, &ident)));
}

fn parse_token_stream<S>(s: S) -> TokenStream
where
    S: AsRef<str> + std::fmt::Display,
{
    s.as_ref()
        .parse::<TokenStream>()
        .unwrap_or_else(|e| panic!("Unable to parse TokenStream: {s}: {e}"))
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
    let variant_count = variants.len() as isize;

    let arbitrary_variants: Vec<syn::Arm> = enums::variant_discriminants(variants)
        .map(|(i, v)| {
            let arbitrary_variant = arbitrary_data_constructor(&v.ident, &v.fields);

            let pat = if i < variant_count - 1 {
                syn::Pat::Lit(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::Lit::Int(proc2::Literal::isize_unsuffixed(i).into()),
                })
            } else {
                let i = syn::Lit::Int(proc2::Literal::isize_unsuffixed(i).into());
                parse_quote! { #i.. }
            };

            parse_quote! {
                #pat => #arbitrary_variant
            }
        })
        .collect();

    let variant_count = syn::Lit::Int(proc2::Literal::isize_unsuffixed(variant_count).into());

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

fn type_path(ident: &Ident) -> Type {
    Type::Path(syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: iter::once(syn::PathSegment {
                ident: ident.clone(),
                arguments: syn::PathArguments::default(),
            })
            .collect(),
        },
    })
}

fn fn_test_layout(symbols: &Symbols, ident: &Ident) -> syn::ItemFn {
    fn_test_layout_of(symbols, ident, &type_path(ident))
}

fn fn_test_layout_of(symbols: &Symbols, ident: &Ident, ty: &Type) -> syn::ItemFn {
    let fn_ident = format_ident!("sys_{}_layout", ident);
    let asserts = assert_layout_of(symbols, ty);

    parse_quote! {
        #[cfg(feature = "sys")]
        #[test]
        fn #fn_ident() {
            #(#asserts)*
        }
    }
}

fn assert_layout_of(symbols: &Symbols, ty: &Type) -> Vec<syn::Stmt> {
    let sys_ty = prefix_with_sys(symbols, ty);

    let block: syn::Block = parse_quote! {
        {
            assert_eq!(size_of::<#ty>(), size_of::<#sys_ty>());
            assert_eq!(align_of::<#ty>(), align_of::<#sys_ty>());
        }
    };

    block.stmts
}

fn fn_test_layout_of_val(
    ident: &Ident,
    safe: bool,
    sys_safe: bool,
    attr: Option<syn::Attribute>,
) -> syn::ItemFn {
    let fn_ident = format_ident!("sys_{}_layout", ident);

    let asserts = assert_layout_of_val(ident, safe, sys_safe);

    parse_quote! {
        #[cfg(feature = "sys")]
        #[test]
        #attr
        fn #fn_ident() {
            #(#asserts)*
        }
    }
}

fn assert_layout_of_val(ident: &Ident, safe: bool, sys_safe: bool) -> Vec<syn::Stmt> {
    let val = if safe {
        quote!(&#ident)
    } else {
        quote!(unsafe { &#ident })
    };

    let sys_val = if sys_safe {
        quote!(&sys::#ident)
    } else {
        quote!(unsafe { &sys::#ident })
    };

    let block: syn::Block = parse_quote! {
        {
            assert_eq!(size_of_val(#val), size_of_val(#sys_val));
            assert_eq!(align_of_val(#val), align_of_val(#sys_val));
        }
    };

    block.stmts
}

fn attr_ffi(consumers: Consumers) -> syn::Attribute {
    let mut cs = String::with_capacity(100);
    let mut is_first = true;

    for p in consumers {
        if !is_first {
            cs.push_str(", ");
        } else {
            is_first = false;
        }
        cs.push_str(p.to_str());
    }
    let cs = parse_token_stream(cs);

    parse_quote! { #[ffi(#cs)] }
}
