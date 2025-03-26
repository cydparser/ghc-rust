use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use proc_macro2::{Span, TokenStream};
use quote::format_ident;
use syn::{parse_quote, punctuated::Punctuated, token, Ident, Item, Visibility};

use generate::InternalSymbols;

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

    let symbols = InternalSymbols::new();

    // Create directories, one for each module, in the destination dir.
    for_each_rs(&src_dir, &|path| {
        eprintln!("Processing {}", path.display());
        let main_rs = dst_dir.join(path.strip_prefix(&src_dir).unwrap());
        let mod_dir = main_rs.with_extension("");

        let code = fs::read_to_string(path).unwrap();
        let Transformed {
            main_file,
            tests_file,
        } = transform_tree(&symbols, syn::parse_file(&code).unwrap());

        fs::create_dir_all(&mod_dir)?;

        for (file, syn_file) in [(main_rs, main_file), (mod_dir.join("tests.rs"), tests_file)] {
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
        padded.push_str("\n");

        if line.starts_with("}")
            || ((line.starts_with("pub") || line.starts_with("static")) && line.ends_with(";"))
            || line == "mod tests;"
        {
            padded.push_str("\n")
        }
    }
    padded
}

fn for_each_rs<F>(dir: &Path, f: &F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&Path) -> Result<(), Box<dyn std::error::Error>>,
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

fn transform_tree(symbols: &InternalSymbols, syn_file: syn::File) -> Transformed {
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
            use std::mem::transmute;
        }),
        Item::Use(parse_quote! {
            #[cfg(feature = "tracing")]
            use tracing::instrument;
        }),
        Item::Use(parse_quote! {
            #[cfg(test)]
            use quickcheck::{Arbitrary, Gen};
        }),
        Item::Use(parse_quote! {
            #[cfg(feature = "sys")]
            use ghc_rts_sys as sys;
        }),
        use_stg_types.clone(),
    ]);

    transformed.tests_file.items.extend([
        Item::Use(parse_quote! {
            use std::mem::{size_of, transmute};
        }),
        Item::Use(parse_quote! {
            use quickcheck::quickcheck;
        }),
        Item::Use(parse_quote! {
            #[cfg(feature = "sys")]
            use ghc_rts_sys as sys;
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
                // Enums are not referrenced outside of C files.
                item_enum.vis = parse_quote! { pub(crate) };
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
                            let (vis, attr) = if symbols.is_internal_static(&ident) {
                                (Visibility::Inherited, None)
                            } else {
                                (vis, Some(parse_token_stream("#[unsafe(no_mangle)]")))
                            };

                            transformed.main_file.items.push(Item::Static(parse_quote! {
                                #attr
                                #vis static #mutability #ident: #ty = sys::#ident;
                            }));
                        }
                        fitem => panic!("Unexpected Item: {:#?}", fitem),
                    }
                }
            }
            Item::Impl(item_impl) => transformed.main_file.items.push(Item::Impl(item_impl)),
            Item::Struct(item_struct) => transform_struct(symbols, item_struct, &mut transformed),
            Item::Type(mut item_type) => {
                if symbols.is_internal_type(&item_type.ident) {
                    item_type.vis = parse_quote! { pub(crate) };
                }
                transformed.main_file.items.push(Item::Type(item_type));
            }
            Item::Union(item_union) => transform_union(item_union, &mut transformed),
            item @ Item::Use(_) => transformed.main_file.items.push(item),
            item => panic!("Unexpected Item: {:#?}", item),
        }
    }

    transformed
}

fn transform_const(
    symbols: &InternalSymbols,
    mut item_const: syn::ItemConst,
    transformed: &mut Transformed,
) {
    let ident = item_const.ident.clone();

    if symbols.is_internal_const(&ident) {
        item_const.vis = parse_quote! { pub(crate) };
    }

    if ident.to_string() == "_" {
        transformed.tests_file.items.push(Item::Const(item_const));
        return;
    } else {
        transformed.main_file.items.push(Item::Const(item_const))
    };

    let test_eq = format_ident!("test_eq_{}", ident);

    transformed.tests_file.items.push(Item::Fn(parse_quote! {
        #[cfg(feature = "sys")]
        #[test]
        fn #test_eq() {
            assert_eq!(sys::#ident, super::#ident.into());
        }
    }));
}

fn transform_ffn(
    symbols: &InternalSymbols,
    ffn: syn::ForeignItemFn,
    transformed: &mut Transformed,
) {
    let syn::ForeignItemFn {
        vis,
        sig:
            syn::Signature {
                abi,
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

    let Transformed {
        main_file,
        tests_file,
    } = transformed;

    // TODO: There are variadic functions in rts::messages.
    if variadic.is_some() {
        eprintln!("Ignoring variadic function: {}", ident);
        return;
    }

    let (inputs_owned, _args, args_into, args_from_owned, bindings): (
        Punctuated<_, token::Comma>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(pat_ident @ syn::PatIdent { .. }) = pat_type.pat.as_ref() {
                    let param_ident = pat_ident.ident.clone();

                    let (ty_owned, arg_into, arg_from_owned) = match pat_type.ty.as_ref() {
                        ty @ syn::Type::Path(_) => (
                            ty.clone(),
                            expr_into(&param_ident),
                            syn::Pat::Ident(new_pat_ident(&param_ident)),
                        ),
                        syn::Type::Ptr(type_ptr) => ptr_to_ty_expr_pat(&param_ident, type_ptr),
                        ty => panic!("Unexpected type in {:?}: {:?}", arg, ty),
                    };

                    let binding: TokenStream =
                        format!("let {} = Default::default();", &param_ident)
                            .parse()
                            .unwrap_or_else(|e| {
                                panic!("Unable to parse let expression: {:?} {}", arg, e)
                            });

                    (
                        syn::FnArg::Typed(syn::PatType {
                            attrs: pat_type.attrs.clone(),
                            pat: pat_type.pat.clone(),
                            colon_token: pat_type.colon_token.clone(),
                            ty: Box::new(ty_owned),
                        }),
                        param_ident,
                        arg_into,
                        arg_from_owned,
                        binding,
                    )
                } else {
                    panic!("Expected only syn::Pat::Ident: {:#?}", arg);
                }
            }
            syn::FnArg::Receiver(_) => {
                panic!("Unexpected FnArg::Recever: {:#?}", arg)
            }
        })
        .collect();

    // Mark all functions as unsafe until the code can be audited.
    let (vis, abi, attr) = if symbols.is_internal_func(&ident) {
        (parse_quote! { pub(crate) }, None, None)
    } else {
        (vis, abi, Some(parse_token_stream("#[unsafe(no_mangle)]")))
    };
    main_file.items.push(Item::Fn(parse_quote! {
        #attr
        #[cfg_attr(feature = "tracing", instrument)]
        #vis unsafe #abi fn #ident(#inputs) #output {
            unsafe { transmute(sys::#ident(#(#args_into),*)) }
        }
    }));

    if let syn::ReturnType::Type(_, _) = output {
        let fn_ident = format_ident!("equivalent_{}", ident);

        tests_file.items.push(Item::Fn(parse_quote! {
            #[cfg(feature = "sys")]
            #[quickcheck]
            fn #fn_ident(#inputs_owned) -> bool {
                let expected = unsafe { transmute(sys::#ident(#(#args_into),*)) };
                super::#ident(#(#args_from_owned),*) == expected
            }
        }));
    }
    let fn_ident = format_ident!("test_{}", ident);

    tests_file.items.push(Item::Fn(parse_quote! {
        #[test]
        #[ignore]
        fn #fn_ident() {
            #(#bindings)*
            super::#ident(#(#args_from_owned),*);
            todo!("assert")
        }
    }));
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

fn ptr_to_ty_expr_pat(ident: &Ident, type_ptr: &syn::TypePtr) -> (syn::Type, syn::Expr, syn::Pat) {
    let (ty, expr, pat) = match type_ptr.elem.as_ref() {
        ty @ syn::Type::Path(_) => (
            ty.clone(),
            expr_into(ident),
            syn::Pat::Ident(new_pat_ident(ident)),
        ),
        syn::Type::Ptr(type_ptr) => ptr_to_ty_expr_pat(ident, type_ptr),
        _ => panic!("Unexpected type for {}: {:?}", ident, type_ptr),
    };

    (
        ty,
        syn::Expr::Reference(syn::ExprReference {
            attrs: vec![],
            and_token: token::And(Span::mixed_site()),
            mutability: type_ptr.mutability,
            expr: Box::new(expr),
        }),
        syn::Pat::Reference(syn::PatReference {
            attrs: vec![],
            and_token: token::And(Span::mixed_site()),
            mutability: type_ptr.mutability,
            pat: Box::new(pat),
        }),
    )
}

fn transform_struct(
    symbols: &InternalSymbols,
    mut item_struct: syn::ItemStruct,
    Transformed {
        main_file,
        tests_file,
    }: &mut Transformed,
) {
    if item_struct.ident.to_string() == "__IncompleteArrayField" {
        main_file.items.push(Item::Struct(item_struct));
        return;
    }
    let ident = item_struct.ident.clone();

    if symbols.is_internal_struct(&ident) {
        item_struct.vis = parse_quote! { pub(crate) };
    }

    let field_idents: Vec<Ident> = match &item_struct.fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
            named.iter().map(|f| f.ident.clone().unwrap()).collect()
        }
        _ => panic!("Unexpected struct type: {:?}", &item_struct),
    };

    let arbitrary_fields: Vec<_> = field_idents
        .iter()
        .cloned()
        .map(|field_ident| parse_token_stream(format!("{}: Arbitrary::arbitrary(g)", field_ident)))
        .collect();

    main_file.items.extend([
        Item::Struct(item_struct),
        Item::Impl(impl_from(&ident)),
        // impl Arbitrary
        Item::Impl(parse_quote! {
            #[cfg(test)]
            impl Arbitrary for #ident {
                fn arbitrary(g: &mut Gen) -> Self {
                    #ident {
                        #(#arbitrary_fields),*
                    }
                }
            }
        }),
    ]);

    tests_file.items.push(Item::Fn(fn_test_size_of(&ident)));
}

fn transform_union(
    mut item_union: syn::ItemUnion,
    Transformed {
        main_file,
        tests_file,
    }: &mut Transformed,
) {
    let ident = item_union.ident.clone();

    item_union.vis = parse_quote! { pub(crate) };

    let field_idents: Vec<Ident> = item_union
        .fields
        .named
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();

    let field_count = field_idents.len();

    let arbitrary_fields: Vec<_> = field_idents
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, field_ident)| {
            parse_token_stream(format!(
                "{} => {} {{ {}: Arbitrary::arbitrary(g) }}",
                i, &ident, field_ident
            ))
        })
        .collect();

    main_file.items.extend([
        Item::Union(item_union),
        Item::Impl(impl_from(&ident)),
        // impl Arbitrary
        Item::Impl(parse_quote! {
            #[cfg(test)]
            impl Arbitrary for #ident {
                fn arbitrary(g: &mut Gen) -> Self {
                    match Arbitrary::arbitrary::<usize>(g) % #field_count {
                        #(#arbitrary_fields),*
                    }
                }
            }
        }),
    ]);

    tests_file.items.push(Item::Fn(fn_test_size_of(&ident)));
}

fn parse_token_stream<S>(s: S) -> TokenStream
where
    S: AsRef<str> + std::fmt::Display,
{
    s.as_ref()
        .parse::<TokenStream>()
        .unwrap_or_else(|e| panic!("Unable to parse TokenStream: {}: {}", s, e))
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

fn fn_test_size_of(ident: &Ident) -> syn::ItemFn {
    let test_size_of = format_ident!("test_size_of_{}", ident);

    parse_quote! {
        #[cfg(feature = "sys")]
        #[test]
        fn #test_size_of() {
            assert_eq!(size_of::<sys::#ident>(), size_of::<super::#ident>())
        }
    }
}
