use proc_macro2::{Span, TokenStream};
use quote::format_ident;
use std::{
    fs,
    path::{Path, PathBuf},
};
use syn::{parse_quote, punctuated::Punctuated, token, Ident, Item, Visibility};

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

    // Create directories, one for each module, in the destination dir.
    for_each_rs(&src_dir, &|path| {
        eprintln!("Processing {}", path.display());
        let main_rs = dst_dir.join(path.strip_prefix(&src_dir).unwrap());
        let mod_dir = main_rs.with_extension("");

        let code = fs::read_to_string(path).unwrap();
        let Transformed {
            main_file,
            tests_file,
        } = transform_tree(syn::parse_file(&code).unwrap());

        fs::create_dir_all(&mod_dir)?;
        fs::write(&main_rs, prettyplease::unparse(&main_file).as_bytes())?;
        fs::write(
            mod_dir.join("tests.rs"),
            prettyplease::unparse(&tests_file).as_bytes(),
        )?;

        Ok(())
    })
    .unwrap();
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

fn transform_tree(syn_file: syn::File) -> Transformed {
    let mut transformed = Transformed {
        main_file: syn::File {
            shebang: None,
            attrs: vec![],
            items: Vec::with_capacity(syn_file.items.len()),
        },
        tests_file: syn::File {
            shebang: None,
            attrs: vec![],
            items: vec![
                Item::Use(parse_quote! { use ghc_rts_sys as sys; }),
                Item::Use(parse_quote! { use quickcheck::quickcheck; }),
            ],
        },
    };
    let mut items = syn_file.items.into_iter().peekable();

    transformed
        .main_file
        .items
        .push(Item::Use(parse_quote! { use ghc_rts_sys as sys; }));

    // Add original imports.
    while let Some(item) = items.peek() {
        match item {
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
            Item::Const(mut item_const) => {
                item_const.vis = Visibility::Inherited;
                transformed.main_file.items.push(Item::Const(item_const));
            }
            Item::ForeignMod(foreign_mod) => {
                for fitem in foreign_mod.items.into_iter() {
                    match fitem {
                        syn::ForeignItem::Fn(ffn) => {
                            transform_ffn(ffn, &mut transformed);
                        }
                        syn::ForeignItem::Static(syn::ForeignItemStatic {
                            ident,
                            mutability,
                            ty,
                            ..
                        }) => {
                            // TODO: Determine witch statics must be exported.
                            transformed.main_file.items.push(Item::Static(parse_quote! {
                                static #mutability #ident: #ty = ghc_rts_sys::#ident;
                            }));
                        }
                        fitem => panic!("Unexpected Item: {:#?}", fitem),
                    }
                }
            }
            Item::Impl(item_impl) => todo!(),
            Item::Macro(item_macro) => todo!(),
            Item::Mod(item_mod) => todo!(),
            Item::Static(item_static) => todo!(),
            Item::Struct(item_struct) => todo!(),
            Item::Trait(item_trait) => todo!(),
            Item::TraitAlias(item_trait_alias) => todo!(),
            Item::Type(item_type) => todo!(),
            Item::Union(item_union) => todo!(),
            item => panic!("Unexpected Item: {:#?}", item),
        }
    }

    transformed
}

fn transform_ffn(ffn: syn::ForeignItemFn, transformed: &mut Transformed) {
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

    let Transformed {
        main_file,
        tests_file,
    } = transformed;

    // TODO: There are variadic functions in rts::messages.
    if variadic.is_some() {
        eprintln!("Ignoring variadic function: {}", ident);
        return;
    }

    let (inputs_owned, args, args_into, args_from_owned, bindings): (
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
    let func = parse_quote! {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn #ident(#inputs) #output {
            unsafe { sys::#ident(#(#args),*) }
        }
    };
    main_file.items.push(Item::Fn(func));
    if let syn::ReturnType::Type(_, _) = output {
        let fn_ident = format_ident!("equivalent_{}", ident);

        tests_file.items.push(Item::Fn(parse_quote! {
            #[cfg(feature = "sys")]
            #[quickcheck]
            fn #fn_ident(#inputs_owned) -> bool {
                let expected = unsafe { sys::#ident(#(#args_into),*) };
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
