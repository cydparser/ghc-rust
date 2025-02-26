use quote::quote;
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
            transform_tree(Consts::new(), syn::parse_file(&code).unwrap());

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

struct Consts {
    use_ghc_rts_sys: Item,
    mod_tests: Item,
}

impl Consts {
    fn new() -> Consts {
        Consts {
            use_ghc_rts_sys: Item::Verbatim(quote! {
                use ghc_rts_sys as sys;
            }),
            mod_tests: Item::Verbatim(quote! {
                #[cfg(test)]
                mod tests;
            }),
        }
    }
}

fn transform_tree(consts: Consts, syn_file: syn::File) -> (syn::File, syn::File) {
    let mut main_file = syn::File {
        shebang: None,
        attrs: vec![],
        items: Vec::with_capacity(syn_file.items.len()),
    };
    let mut items = syn_file.items.into_iter().peekable();

    main_file.items.push(consts.use_ghc_rts_sys.clone());
    // Add original imports.
    while let Some(item) = items.peek() {
        match item {
            Item::Use(_) => main_file.items.push(items.next().unwrap()),
            _ => break,
        }
    }
    main_file.items.push(consts.mod_tests.clone());

    // TODO
    // [ ] fn
    //     #[unsafe(no_mangle)]
    //     extern "C" fn foo(args*) -> ret {
    //         unsafe { sys::foo(args) }
    //     }

    for item in items {
        match item {
            Item::Const(item_const) => todo!(),
            Item::Enum(item_enum) => todo!(),
            Item::ExternCrate(item_extern_crate) => todo!(),
            Item::Fn(item_fn) => todo!(),
            Item::ForeignMod(item_foreign_mod) => todo!(),
            Item::Impl(item_impl) => todo!(),
            Item::Macro(item_macro) => todo!(),
            Item::Mod(item_mod) => todo!(),
            Item::Static(item_static) => todo!(),
            Item::Struct(item_struct) => todo!(),
            Item::Trait(item_trait) => todo!(),
            Item::TraitAlias(item_trait_alias) => todo!(),
            Item::Type(item_type) => todo!(),
            Item::Union(item_union) => todo!(),
            Item::Use(item_use) => panic!("Unexpected Use: {:?}", item_use),
            Item::Verbatim(token_stream) => panic!("Unexpected Verbatim: {:?}", token_stream),
            _ => todo!(),
        }
    }

    let tests_file = syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![consts.use_ghc_rts_sys.clone()],
    };

    (main_file, tests_file)
}
