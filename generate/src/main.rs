use quote::{format_ident, quote};
use std::{
    fs,
    path::{Path, PathBuf},
};
use syn::{token::PathSep, Ident, Item, Token, UseName, UsePath, UseTree};

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
        let (main_file, tests_file) =
            transform_tree(syn::parse_file(&code).unwrap());

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

fn transform_tree(syn_file: syn::File) -> (syn::File, syn::File) {
    let mut main_file = syn::File {
        shebang: None,
        attrs: vec![],
        items: Vec::with_capacity(syn_file.items.len()),
    };
    let mut items = syn_file.items.into_iter().peekable();

    main_file
        .items
        .push(Item::Use(parse_quote! { use ghc_rts_sys as sys; }));
    // Add original imports.
    while let Some(item) = items.peek() {
        match item {
            Item::Use(_) => main_file.items.push(items.next().unwrap()),
            _ => break,
        }
    }
    main_file.items.push(Item::Mod(parse_quote! {
        #[cfg(test)]
        mod tests;
    }));

    let mut tests_file = syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![
            Item::Use(parse_quote! { use ghc_rts_sys as sys; }),
            Item::Use(parse_quote! { use quickcheck::quickcheck; }),
        ],
    };

    for item in items {
        match item {
            Item::Const(mut item_const) => {
                item_const.vis = Visibility::Inherited;
                main_file.items.push(Item::Const(item_const));
            }
            Item::ForeignMod(foreign_mod) => {
                for fitem in foreign_mod.items.into_iter() {
                    match fitem {
                        syn::ForeignItem::Fn(syn::ForeignItemFn {
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
                        }) => {
                            assert!(generics.gt_token.is_none() && generics.where_clause.is_none());

                            // TODO: There are variadic functions in rts::messages.
                            if variadic.is_some() {
                                eprintln!("Ignoring variadic function: {}", ident);
                                continue;
                            }
                            let args: Vec<Ident> = inputs
                                .iter()
                                .map(|arg| match arg {
                                    syn::FnArg::Typed(syn::PatType { pat, .. }) => {
                                        if let syn::Pat::Ident(
                                            ref pat_ident @ syn::PatIdent { .. },
                                        ) = **pat
                                        {
                                            pat_ident.ident.clone()
                                        } else {
                                            panic!(
                                                "Expected only syn::Pat::Ident variants: {:}",
                                                &ident
                                            );
                                        }
                                    }
                                    syn::FnArg::Receiver(_) => {
                                        panic!("Unexpected FnArg::Recever: {:}", &ident)
                                    }
                                })
                                .collect();

                            let func = parse_quote! {
                                #[unsafe(no_mangle)]
                                pub extern "C" fn #ident(#inputs) #output {
                                    unsafe { sys::#ident(#(#args),*) }
                                }
                            };
                            main_file.items.push(Item::Fn(func));
                            if let syn::ReturnType::Type(_, _) = output {
                                let fn_ident = format_ident!("equivalent_{}", ident);

                                // TODO: Convert types using trait like To
                                tests_file.items.push(Item::Fn(parse_quote! {
                                    #[cfg(feature = "sys")]
                                    #[quickcheck]
                                    fn #fn_ident(#inputs) -> bool {
                                        let expected = unsafe { sys::#ident(#(#args),*) };
                                        super::#ident(#(#args),*) == expected
                                    }
                                }));
                            } else {
                                // TODO: Should this still be a property test?
                                let fn_ident = format_ident!("test_{}", ident);

                                tests_file.items.push(Item::Fn(parse_quote! {
                                    #[test]
                                    fn #fn_ident() {
                                        super::#ident(#(#args),*);
                                        todo!()
                                    }
                                }));
                            }
                        }
                        syn::ForeignItem::Static(syn::ForeignItemStatic {
                            ident,
                            mutability,
                            ty,
                            ..
                        }) => {
                            // NB: Statics are not exported (#[unsafe(no_mangle)]) because they are not thread safe.
                            main_file.items.push(Item::Static(parse_quote! {
                                pub(crate) static #mutability #ident: #ty = ghc_rts_sys::#ident;
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

    (main_file, tests_file)
}
