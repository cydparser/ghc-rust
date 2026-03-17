use generate_refactor::{args_rs, format, item_ident};
use proc_macro2::Span;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::path::{self, Path, PathBuf};
use std::{fs, iter};
use syn::{Attribute, Token, UseTree};
use syn::{Ident, Item, ItemUse, Visibility, parse_quote, punctuated::Punctuated};

type Result<T> = std::result::Result<T, Box<dyn Error + 'static>>;

fn main() -> Result<()> {
    let rts_src_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("rts/src");

    let paths = args_rs()?;

    let mut context = Context::new(&paths, &rts_src_dir)?;

    for path in paths {
        eprintln!("  * Transforming {path:?}");

        context.file_context = FileContext::new(path.as_path())?;

        let syn_file = {
            let syn_file = parse_syn_file(&path)?;

            transform(&mut context, false, syn_file)?
        };

        fs::write(path, format(syn_file).as_bytes())?;
    }

    transform_ffi(&rts_src_dir, context)?;

    Ok(())
}

struct Context {
    file_context: FileContext,
    generated_headers: BTreeSet<Ident>,
    header_modules: HashMap<HeaderModuleName, ModulePath>,
    extern_ident_modules: HashMap<Ident, &'static ModulePath>,
    ffi_items: HashMap<Ident, (&'static Path, FfiItem)>,
    test_items: HashMap<Ident, Item>,
    reexports: HashMap<&'static Path, BTreeMap<&'static ModulePath, Vec<Ident>>>,
}

struct FileContext {
    module_name: HeaderModuleName,
    module_path: &'static ModulePath,
    extern_imports: HashMap<&'static ModulePath, Vec<Ident>>,
}

impl FileContext {
    fn new(path: &Path) -> Result<FileContext> {
        let filepath = path.to_string_lossy();

        Ok(FileContext {
            module_name: <HeaderModuleName as TryFrom<&Path>>::try_from(path)?,
            module_path: {
                let (_, mod_filepath) = filepath
                    .rsplit_once("/src/")
                    .ok_or_else(|| format!("filepath missing an ancestor 'src' dir: {filepath}"))?;
                Box::leak(Box::new(ModulePath::from_filepath(mod_filepath)))
            },
            extern_imports: HashMap::new(),
        })
    }
}

impl Context {
    fn new(paths: &[PathBuf], rts_src_dir: &Path) -> Result<Context> {
        let ffi_dir = rts_src_dir.join("ffi");
        let mut ffi_items: HashMap<Ident, (&'static Path, FfiItem)> = HashMap::with_capacity(1028);
        let mut test_items: HashMap<Ident, Item> = HashMap::with_capacity(1028);
        let mut header_modules = HashMap::with_capacity(128);

        let extern_ident_modules = {
            let mut extern_idents: HashSet<Ident> = HashSet::with_capacity(128);
            let mut ident_modules: HashMap<Ident, &'static ModulePath> =
                HashMap::with_capacity(1028);

            for path in paths {
                let FileContext {
                    module_name,
                    module_path,
                    ..
                } = FileContext::new(path)?;

                let syn_file = parse_syn_file(path)?;

                for item in syn_file.items {
                    if let Some(ident) = item_ident(&item) {
                        ident_modules.insert(ident, module_path);
                    } else {
                        match item {
                            Item::ForeignMod(item_foreign_mod) => {
                                for fitem in item_foreign_mod.items {
                                    if let Some(ident) = foreign_item_ident(&fitem) {
                                        extern_idents.insert(ident);
                                    }
                                }
                            }
                            Item::Mod(item_mod) if module_name.should_inline(&item_mod.ident) => {
                                for mitem in
                                    item_mod.content.into_iter().flat_map(|(_, items)| items)
                                {
                                    if let Some(ident) = item_ident(&mitem) {
                                        ident_modules.insert(ident, module_path);
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }

            let mut extern_ident_modules = HashMap::with_capacity(extern_idents.len());

            for ident in extern_idents {
                if let Some(&module) = ident_modules.get(&ident) {
                    extern_ident_modules.insert(ident, module);
                }
            }

            extern_ident_modules
        };

        let push_aux_item = |ffi_items: &mut HashMap<Ident, (&'static Path, FfiItem)>,
                             ident: &Ident,
                             item|
         -> Result<()> {
            let (_, ffi_item) = ffi_items
                .get_mut(ident)
                .ok_or_else(|| format!("push_aux_item missing item: {ident}"))?;

            ffi_item.push_aux_item(item)
        };

        let is_anon_data = |ident: &Ident| -> bool {
            let name = ident.to_string();
            // The FFI symbols do not contain double underscores, but bindgen ones do.
            name.contains("__")
        };

        #[allow(clippy::type_complexity)]
        let anon_data = |ffi_items: &mut HashMap<Ident, (&'static Path, FfiItem)>,
                         ident: &Ident,
                         prev_data_ident,
                         item|
         -> Result<(Option<Ident>, Option<(&'static Path, FfiItem)>)> {
            if let Some(prev_ident) = &prev_data_ident {
                push_aux_item(ffi_items, prev_ident, item)?;
            } else {
                Err(format!(
                    "expected previous data type for anonomous bindgen type: {ident}"
                ))?;
            }
            Ok((prev_data_ident, None))
        };

        for_each_rs_file(ffi_dir.as_path(), &mut |path| {
            if path.file_name().unwrap() == "tests.rs" {
                for item in parse_syn_file(path)?.items {
                    if let Some(ident) = item_ident(&item) {
                        test_items.insert(ident, item);
                    }
                }
            } else {
                let relpath: &'static Path = path.strip_prefix(rts_src_dir)?.to_owned().leak();
                let filepath = relpath.to_string_lossy().to_string();

                let mut prev_data_ident: Option<Ident> = None;

                let module_name = HeaderModuleName::try_from(relpath)?;

                let module_path = ModulePath::from_filepath(&filepath);

                header_modules.insert(module_name, module_path);

                for item in parse_syn_file(path)?.items {
                    let (data_ident, insert_result): (
                        Option<Ident>,
                        Option<(&'static Path, FfiItem)>,
                    ) = match item {
                        Item::Const(item) => (
                            None,
                            ffi_items.insert(
                                item.ident.clone(),
                                (relpath, FfiItem::Const(item, vec![])),
                            ),
                        ),
                        Item::Enum(item) => {
                            let ident = item.ident.clone();
                            (
                                Some(ident.clone()),
                                ffi_items.insert(ident, (relpath, FfiItem::Enum(item, vec![]))),
                            )
                        }
                        Item::Fn(syn::ItemFn {
                            attrs, vis, sig, ..
                        }) => (
                            None,
                            ffi_items.insert(
                                sig.ident.clone(),
                                (relpath, FfiItem::Fn { attrs, vis, sig }),
                            ),
                        ),
                        Item::Impl(item) => {
                            if let Some(prev_ident) = prev_data_ident.as_ref() {
                                push_aux_item(&mut ffi_items, prev_ident, Item::Impl(item))?;
                            }
                            (prev_data_ident, None)
                        }
                        Item::Static(item) => (
                            None,
                            ffi_items.insert(item.ident.clone(), (relpath, FfiItem::Static(item))),
                        ),
                        Item::Struct(item) => {
                            let ident = item.ident.clone();

                            if is_anon_data(&ident) {
                                anon_data(
                                    &mut ffi_items,
                                    &ident,
                                    prev_data_ident,
                                    Item::Struct(item),
                                )?
                            } else {
                                (
                                    Some(ident.clone()),
                                    ffi_items
                                        .insert(ident, (relpath, FfiItem::Struct(item, vec![]))),
                                )
                            }
                        }
                        Item::Type(item) => (
                            prev_data_ident,
                            ffi_items.insert(item.ident.clone(), (relpath, FfiItem::Type(item))),
                        ),
                        Item::Union(item) => {
                            let ident = item.ident.clone();

                            if is_anon_data(&ident) {
                                anon_data(
                                    &mut ffi_items,
                                    &ident,
                                    prev_data_ident,
                                    Item::Union(item),
                                )?
                            } else {
                                (
                                    Some(ident.clone()),
                                    ffi_items
                                        .insert(ident, (relpath, FfiItem::Union(item, vec![]))),
                                )
                            }
                        }
                        _ => (None, None),
                    };

                    prev_data_ident = data_ident;
                    insert_result.iter().for_each(|(path, ffi_item)| {
                        eprintln!(
                            "ident {} already present in {}",
                            ffi_item.ident(),
                            path.display()
                        );
                    });
                }
            }

            Ok(())
        })?;

        static EMPTY_MODULE_PATH: ModulePath = ModulePath(vec![]);

        Ok(Context {
            file_context: FileContext {
                module_name: HeaderModuleName(String::new()),
                module_path: &EMPTY_MODULE_PATH,
                extern_imports: HashMap::new(),
            },
            header_modules,
            generated_headers: BTreeSet::new(),
            extern_ident_modules,
            ffi_items,
            test_items,
            reexports: HashMap::new(),
        })
    }

    fn visibility(&self, in_header: bool, ident: &Ident) -> Visibility {
        if in_header || self.extern_ident_modules.contains_key(ident) {
            parse_quote! { pub(crate) }
        } else {
            Visibility::Inherited
        }
    }

    fn add_reexport(&mut self, key: &'static Path, ident: Ident) {
        let module_idents = self.reexports.entry(key).or_default();
        module_idents
            .entry(self.file_context.module_path)
            .or_default()
            .push(ident)
    }
}

#[derive(Debug)]
enum FfiItem {
    Const(syn::ItemConst, Vec<syn::Item>),
    Enum(syn::ItemEnum, Vec<syn::Item>),
    Fn {
        attrs: Vec<Attribute>,
        vis: Visibility,
        sig: syn::Signature,
    },
    Static(syn::ItemStatic),
    Struct(syn::ItemStruct, Vec<syn::Item>),
    Type(syn::ItemType),
    Union(syn::ItemUnion, Vec<syn::Item>),
}

impl FfiItem {
    fn push_aux_item(&mut self, item: Item) -> Result<()> {
        let items = match self {
            FfiItem::Const(_, items) => Ok(items),
            FfiItem::Enum(_, items) => Ok(items),
            FfiItem::Fn { sig, .. } => Err(format!("no items for {}", sig.ident)),
            FfiItem::Static(item) => Err(format!("no items for {}", item.ident)),
            FfiItem::Struct(_, items) => Ok(items),
            FfiItem::Type(item) => Err(format!("no items for {}", item.ident)),
            FfiItem::Union(_, items) => Ok(items),
        }?;
        items.push(item);

        Ok(())
    }

    fn ident(&self) -> &Ident {
        match self {
            FfiItem::Const(item, _) => &item.ident,
            FfiItem::Enum(item, _) => &item.ident,
            FfiItem::Fn { sig, .. } => &sig.ident,
            FfiItem::Static(item) => &item.ident,
            FfiItem::Struct(item, _) => &item.ident,
            FfiItem::Type(item) => &item.ident,
            FfiItem::Union(item, _) => &item.ident,
        }
    }

    fn is_public(&self) -> bool {
        let attrs = match self {
            FfiItem::Const(item, _) => &item.attrs,
            FfiItem::Enum(item, _) => &item.attrs,
            FfiItem::Fn { attrs, .. } => attrs,
            FfiItem::Static(item) => &item.attrs,
            FfiItem::Struct(item, _) => &item.attrs,
            FfiItem::Type(item) => &item.attrs,
            FfiItem::Union(item, _) => &item.attrs,
        };

        attrs.iter().any(|attr| {
            attr.path()
                .segments
                .first()
                .is_some_and(|s| s.ident == "ffi")
        })
    }

    fn into_items(self, item: Item) -> Result<Vec<Item>> {
        match (item, self) {
            (Item::Const(mut item), FfiItem::Const(ffi, impls)) => {
                item.vis = ffi.vis;
                item.attrs = ffi.attrs;
                Ok(iter::once(Item::Const(item)).chain(impls).collect())
            }
            (Item::Enum(mut item), FfiItem::Enum(ffi, impls)) => {
                item.vis = ffi.vis;
                item.attrs = ffi.attrs;
                Ok(iter::once(Item::Enum(item)).chain(impls).collect())
            }
            (Item::Fn(mut item), FfiItem::Fn { attrs, vis, .. }) => {
                item.vis = vis;
                item.attrs = attrs;
                Ok(vec![Item::Fn(item)])
            }
            (Item::Static(mut item), FfiItem::Static(ffi)) => {
                item.vis = ffi.vis;
                item.attrs = ffi.attrs;
                Ok(vec![Item::Static(item)])
            }
            (Item::Struct(mut item), FfiItem::Struct(ffi, impls)) => {
                item.vis = ffi.vis;
                item.attrs = ffi.attrs;
                Ok(iter::once(Item::Struct(item)).chain(impls).collect())
            }
            (Item::Type(mut item), FfiItem::Type(ffi)) => {
                item.vis = ffi.vis;
                item.attrs = ffi.attrs;
                Ok(vec![Item::Type(item)])
            }
            (Item::Union(mut item), FfiItem::Union(ffi, impls)) => {
                item.vis = ffi.vis;
                item.attrs = ffi.attrs;
                Ok(iter::once(Item::Union(item)).chain(impls).collect())
            }
            (item, ffi_item) => Err(format!("unable to compute items: {:?}", (item, ffi_item)))?,
        }
    }
}

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct HeaderModuleName(String);

impl From<&Ident> for HeaderModuleName {
    fn from(mod_ident: &Ident) -> Self {
        HeaderModuleName::from(mod_ident.to_string())
    }
}

impl From<String> for HeaderModuleName {
    fn from(mut mod_name: String) -> Self {
        debug_assert!(mod_name.ends_with("_h"));
        mod_name.truncate(mod_name.len() - 2); // Strip "_h" suffix.
        mod_name.make_ascii_lowercase();

        HeaderModuleName(mod_name)
    }
}

impl TryFrom<&Path> for HeaderModuleName {
    type Error = String;

    fn try_from(path: &Path) -> std::result::Result<Self, Self::Error> {
        let name = {
            let name = path
                .file_stem()
                .ok_or_else(|| format!("missing filename: {path:?}"))?
                .to_string_lossy();

            if name.chars().next().is_some_and(char::is_uppercase) {
                name.to_string()
            } else {
                stringcase::pascal_case(name.as_ref())
            }
        };
        let is_ffi = path
            .components()
            .next()
            .is_some_and(|c| matches!(c, path::Component::Normal(s) if s == "ffi"));

        let mut mod_name = String::with_capacity(20);

        match (is_ffi, name.as_str()) {
            (true, "Messages") => mod_name.push_str("rts_Messages"),
            (true, "NonMoving") => mod_name.push_str("rts_NonMoving"),
            (true, "Ticky") => mod_name.push_str("rts_Ticky"),
            (true, "Types") => mod_name.push_str("stg_Types"),
            (false, "ForeignExports") => mod_name.push_str("rts_ForeignExports"),
            (false, "StableName") => mod_name.push_str("rts_StableName"),
            (false, "StablePtr") => mod_name.push_str("rts_StablePtr"),
            (false, "Threads") => mod_name.push_str("rts_Threads"),
            (false, "Timer") => mod_name.push_str("rts_Timer"),
            (false, "GC") => mod_name.push_str("sm_GC"),
            _ => {
                mod_name.push_str(name.as_ref());
            }
        };
        mod_name.make_ascii_lowercase();

        Ok(HeaderModuleName(mod_name))
    }
}

impl HeaderModuleName {
    fn should_inline(&self, mod_ident: &Ident) -> bool {
        let other = HeaderModuleName::from(mod_ident);

        *self == other || {
            let prefix = other.0;

            self.0 == format!("{prefix}internal_h") || self.0 == format!("{prefix}internals_h")
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct ModulePath(Vec<String>);

impl ModulePath {
    fn from_filepath(filepath: &str) -> ModulePath {
        let filepath = match filepath.split_once('.') {
            Some((s, _ext)) => s,
            None => filepath,
        };

        let mut parts = Vec::with_capacity(4);

        for part in filepath.split('/') {
            parts.push(if part.chars().next().is_some_and(char::is_uppercase) {
                stringcase::snake_case(part)
            } else {
                part.to_owned()
            });
        }

        ModulePath(parts)
    }

    fn to_item_use(&self, idents: Vec<Ident>) -> syn::ItemUse {
        let use_group = syn::UseGroup {
            brace_token: Default::default(),
            items: idents
                .into_iter()
                .map(|ident| syn::UseTree::Name(syn::UseName { ident }))
                .collect(),
        };

        syn::ItemUse {
            attrs: vec![],
            vis: Visibility::Inherited,
            use_token: Default::default(),
            leading_colon: None,
            tree: self.to_use_tree(use_group),
            semi_token: Default::default(),
        }
    }

    fn to_use_tree(&self, use_group: syn::UseGroup) -> syn::UseTree {
        let tree = self
            .0
            .iter()
            .rfold(syn::UseTree::Group(use_group), |tree, path| {
                syn::UseTree::Path(syn::UsePath {
                    ident: Ident::new(path, Span::call_site()),
                    colon2_token: Default::default(),
                    tree: Box::new(tree),
                })
            });
        syn::UseTree::Path(syn::UsePath {
            ident: Ident::new("crate", Span::call_site()),
            colon2_token: Default::default(),
            tree: Box::new(tree),
        })
    }
}

struct Transformed {
    imports: Vec<ItemUse>,
    items: Vec<Item>,
    test_items: Vec<Item>,
}

fn transform(context: &mut Context, in_header: bool, syn_file: syn::File) -> Result<syn::File> {
    let mut transformed = Transformed {
        imports: vec![parse_quote! {
            use crate::prelude::*;
        }],
        items: vec![],
        test_items: vec![],
    };

    for item in syn_file.items {
        transform_item(context, in_header, &mut transformed, item)?;
    }
    // TODO: Visit items to determine which Idents need to be imported.

    let extern_imports: Vec<syn::ItemUse> = context
        .file_context
        .extern_imports
        .drain()
        .map(|(module_path, idents)| module_path.to_item_use(idents))
        .collect();

    Ok(syn::File {
        shebang: None,
        attrs: vec![],
        items: transformed
            .imports
            .into_iter()
            .chain(extern_imports)
            .map(Item::Use)
            .chain(transformed.items)
            .collect(),
    })
}

fn transform_item(
    context: &mut Context,
    in_header: bool,
    transformed: &mut Transformed,
    item: Item,
) -> Result<()> {
    match item {
        Item::Const(item_const) => transform_const(context, in_header, transformed, item_const),
        Item::Enum(item_enum) => transform_enum(context, in_header, transformed, item_enum),
        Item::Fn(item_fn) => transform_fn(context, in_header, transformed, item_fn),
        Item::ForeignMod(item_foreign_mod) => {
            transform_foreign_mod(context, transformed, item_foreign_mod)
        }
        Item::Impl(item_impl) => panic!("unexpected item_impl {item_impl:?}"),
        Item::Macro(item_macro) => panic!("unexpected item_macro {item_macro:?}"),
        Item::Mod(item_mod) => transform_mod(context, transformed, item_mod),
        Item::Static(item_static) => transform_static(context, in_header, transformed, item_static),
        Item::Struct(item_struct) => transform_struct(context, in_header, transformed, item_struct),
        Item::Trait(item_trait) => panic!("unexpected item_trait {item_trait:?}"),
        Item::TraitAlias(item_trait_alias) => {
            panic!("unexpected item_trait_alias {item_trait_alias:?}")
        }
        Item::Type(item_type) => transform_type(context, in_header, transformed, item_type),
        Item::Union(item_union) => transform_union(context, in_header, transformed, item_union),
        Item::Use(item_use) => transform_use(context, transformed, item_use),
        _ => Ok(()),
    }
}

fn transform_const(
    context: &mut Context,
    in_header: bool,
    transformed: &mut Transformed,
    mut item: syn::ItemConst,
) -> Result<()> {
    if let Some((key, ffi_item)) = context.ffi_items.remove(&item.ident) {
        context.add_reexport(key, item.ident.clone());
        transformed
            .items
            .extend(ffi_item.into_items(Item::Const(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.ident);
        remove_data_attributes(&mut item.attrs);
        transformed.items.push(Item::Const(item));
    }

    Ok(())
}

fn transform_enum(
    context: &mut Context,
    in_header: bool,
    transformed: &mut Transformed,
    mut item: syn::ItemEnum,
) -> Result<()> {
    if let Some((key, ffi_item)) = context.ffi_items.remove(&item.ident) {
        context.add_reexport(key, item.ident.clone());

        for variant in item.variants.iter_mut() {
            set_field_visibility(ffi_item.is_public(), in_header, &mut variant.fields);
        }
        transformed
            .items
            .extend(ffi_item.into_items(Item::Enum(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.ident);

        for variant in item.variants.iter_mut() {
            set_field_visibility(false, in_header, &mut variant.fields);
        }
        remove_data_attributes(&mut item.attrs);
        transformed.items.push(Item::Enum(item));
    }

    Ok(())
}

fn transform_fn(
    context: &mut Context,
    in_header: bool,
    transformed: &mut Transformed,
    mut item: syn::ItemFn,
) -> Result<()> {
    let ident = item.sig.ident.clone();

    if let Some((key, ffi_item)) = context.ffi_items.remove(&ident) {
        context.add_reexport(key, ident);
        transformed
            .items
            .extend(ffi_item.into_items(Item::Fn(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.sig.ident);
        item.sig.abi = None;
        remove_attributes(&mut item.attrs, &[]);
        transformed.items.push(Item::Fn(item));
    }

    Ok(())
}

fn transform_foreign_mod(
    context: &mut Context,
    transformed: &mut Transformed,
    mut item_foreign_mod: syn::ItemForeignMod,
) -> Result<()> {
    item_foreign_mod.items.retain(|fitem| {
        if let Some(ident) = foreign_item_ident(fitem) {
            if let Some(&module_path) = context.extern_ident_modules.get(&ident) {
                context
                    .file_context
                    .extern_imports
                    .entry(module_path)
                    .or_default()
                    .push(ident);

                false
            } else {
                true
            }
        } else {
            // Remove things like imports.
            false
        }
    });

    if !item_foreign_mod.items.is_empty() {
        transformed.items.push(Item::ForeignMod(item_foreign_mod));
    }

    Ok(())
}

fn transform_mod(
    context: &mut Context,
    transformed: &mut Transformed,
    mut item_mod: syn::ItemMod,
) -> Result<()> {
    let module_name = &context.file_context.module_name;

    if module_name.should_inline(&item_mod.ident) {
        for item in item_mod.content.map_or(vec![], |(_, items)| items) {
            transform_item(context, true, transformed, item)?;
        }
    } else if let Some(filepath) = item_mod
        .attrs
        .iter()
        .find(|attr| {
            attr.path()
                .segments
                .iter()
                .map(|s| &s.ident)
                .eq(["c2rust", "header_src"])
        })
        .map(|attr| {
            // Using Meta::List after converting the transpiled files from key value form.
            let args = attr
                .parse_args_with(Punctuated::<syn::Lit, Token![,]>::parse_terminated)
                .map_err(|err| format!("expected MetaList in attr {attr:?}: {err}"))?;

            match args.into_iter().next() {
                Some(syn::Lit::Str(s)) => Ok(s.value()),
                _ => Err(format!("expected string in meta list: {attr:?}")),
            }
        })
        .transpose()?
    {
        // Ignore any headers external to GHC.
        if let [_, relpath] = *filepath.split("/ghc/").collect::<Vec<_>>() {
            if relpath.starts_with("_build/") {
                // The header was generated and will need to be manually transpiled.
                item_mod.vis = Visibility::Inherited;
                if let Some((_, mut items)) = item_mod.content.take() {
                    items.retain_mut(|item| {
                        match item {
                            Item::Const(item) => {
                                item.vis = Visibility::Inherited;
                                remove_attributes(&mut item.attrs, &[]);
                            }
                            Item::Enum(item) => {
                                item.vis = Visibility::Inherited;
                                remove_attributes(&mut item.attrs, &[]);
                            }
                            Item::Fn(item) => {
                                item.vis = Visibility::Inherited;
                                remove_attributes(&mut item.attrs, &[]);
                            }
                            Item::Static(item) => {
                                item.vis = Visibility::Inherited;
                                remove_attributes(&mut item.attrs, &[]);
                            }
                            Item::Struct(item) => {
                                item.vis = Visibility::Inherited;
                                remove_attributes(&mut item.attrs, &[]);
                            }
                            Item::Trait(item) => {
                                item.vis = Visibility::Inherited;
                                remove_attributes(&mut item.attrs, &[]);
                            }
                            Item::Use(_) => return false,
                            Item::Type(item) => {
                                item.vis = Visibility::Inherited;
                                remove_attributes(&mut item.attrs, &[]);
                            }
                            Item::Union(item) => {
                                item.vis = Visibility::Inherited;
                                remove_attributes(&mut item.attrs, &[]);
                            }
                            _ => (),
                        }
                        true
                    });
                }
                remove_attributes(&mut item_mod.attrs, &["c2rust"]);
                context.generated_headers.insert(item_mod.ident.clone());
                transformed.items.push(syn::Item::Mod(item_mod));
                return Ok(());
            }

            let relpath = relpath
                .strip_prefix("rts/")
                .ok_or_else(|| format!("expected a file in rts/: {filepath}"))?;

            let (is_ffi, relpath) = match relpath.strip_prefix("include/") {
                Some(relpath) => (true, relpath),
                None => (false, relpath),
            };

            let mut module_path = ModulePath::from_filepath(relpath);

            if is_ffi {
                module_path.0.insert(0, "ffi".to_string());
            }

            context
                .header_modules
                .insert(HeaderModuleName::from(&item_mod.ident), module_path);
        }
    }

    Ok(())
}

fn transform_static(
    context: &mut Context,
    in_header: bool,
    transformed: &mut Transformed,
    mut item: syn::ItemStatic,
) -> Result<()> {
    if let Some((key, ffi_item)) = context.ffi_items.remove(&item.ident) {
        context.add_reexport(key, item.ident.clone());
        transformed
            .items
            .extend(ffi_item.into_items(Item::Static(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.ident);
        remove_attributes(&mut item.attrs, &[]);
        transformed.items.push(Item::Static(item));
    }

    Ok(())
}

fn transform_struct(
    context: &mut Context,
    in_header: bool,
    transformed: &mut Transformed,
    mut item: syn::ItemStruct,
) -> Result<()> {
    if let Some((key, ffi_item)) = context.ffi_items.remove(&item.ident) {
        context.add_reexport(key, item.ident.clone());
        set_field_visibility(ffi_item.is_public(), in_header, &mut item.fields);
        // TODO: Check for necessary tests in aux items as well
        transformed
            .items
            .extend(ffi_item.into_items(Item::Struct(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.ident);
        set_field_visibility(false, in_header, &mut item.fields);
        remove_data_attributes(&mut item.attrs);
        transformed.items.push(Item::Struct(item));
    }

    Ok(())
}

fn transform_type(
    context: &mut Context,
    in_header: bool,
    transformed: &mut Transformed,
    mut item: syn::ItemType,
) -> Result<()> {
    if let Some((key, ffi_item)) = context.ffi_items.remove(&item.ident) {
        context.add_reexport(key, item.ident.clone());
        transformed
            .items
            .extend(ffi_item.into_items(Item::Type(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.ident);
        remove_data_attributes(&mut item.attrs);
        transformed.items.push(Item::Type(item));
    }

    Ok(())
}

fn transform_union(
    context: &mut Context,
    in_header: bool,
    transformed: &mut Transformed,
    mut item: syn::ItemUnion,
) -> Result<()> {
    if let Some((key, ffi_item)) = context.ffi_items.remove(&item.ident) {
        context.add_reexport(key, item.ident.clone());
        set_named_field_visibility(ffi_item.is_public(), in_header, &mut item.fields);
        transformed
            .items
            .extend(ffi_item.into_items(Item::Union(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.ident);
        set_named_field_visibility(false, in_header, &mut item.fields);
        remove_data_attributes(&mut item.attrs);
        transformed.items.push(Item::Union(item));
    }

    Ok(())
}

fn transform_use(
    context: &mut Context,
    transformed: &mut Transformed,
    mut item: ItemUse,
) -> Result<()> {
    if let UseTree::Path(use_path) = item.tree
        && (use_path.ident == "self" || use_path.ident == "super")
        && let UseTree::Path(use_path) = *use_path.tree
    {
        let ident = &use_path.ident;

        if let Some(module_path) = context.header_modules.get(&HeaderModuleName::from(ident)) {
            let mut use_tree = *use_path.tree;
            let use_group = loop {
                match use_tree {
                    UseTree::Path(use_path) => {
                        use_tree = *use_path.tree;
                    }
                    UseTree::Group(use_group) => {
                        break use_group;
                    }
                    UseTree::Name(use_name) => {
                        break syn::UseGroup {
                            brace_token: Default::default(),
                            items: iter::once(UseTree::Name(use_name)).collect(),
                        };
                    }
                    _ => Err(format!("unexpected UseTree: {use_tree:?}"))?,
                }
            };

            transformed.imports.push(syn::ItemUse {
                attrs: vec![],
                vis: Visibility::Inherited,
                use_token: Default::default(),
                leading_colon: None,
                tree: module_path.to_use_tree(use_group),
                semi_token: Default::default(),
            })
        } else if context.generated_headers.contains(ident) {
            item.tree = UseTree::Path(use_path);
            item.vis = Visibility::Inherited;
            transformed.items.push(syn::Item::Use(item));
        }
    }

    Ok(())
}

fn set_field_visibility(public_ffi: bool, in_header: bool, fields: &mut syn::Fields) {
    match fields {
        syn::Fields::Named(fields_named) => {
            set_named_field_visibility(public_ffi, in_header, fields_named)
        }
        syn::Fields::Unnamed(fields_unnamed) => fields_unnamed
            .unnamed
            .iter_mut()
            .for_each(|f| f.vis = Visibility::Inherited),
        syn::Fields::Unit => (),
    }
}

fn set_named_field_visibility(public_ffi: bool, in_header: bool, fields: &mut syn::FieldsNamed) {
    let vis = if public_ffi {
        Visibility::Public(Default::default())
    } else if in_header {
        parse_quote! { pub(crate) }
    } else {
        Visibility::Inherited
    };

    fields.named.iter_mut().for_each(|f| f.vis = vis.clone())
}

fn remove_attributes(attrs: &mut Vec<Attribute>, remove: &[&str]) {
    attrs.retain(|attr| {
        let ident = &attr.path().segments.first().unwrap().ident;
        // Remove 'no_mangle': it will be replaced by 'unsafe(no_mangle)'.
        ident != "no_mangle" && ident != "c2rust" && remove.iter().all(|name| ident != name)
    });
}

fn remove_data_attributes(attrs: &mut Vec<Attribute>) {
    // Remove 'derive(Copy, Clone)': these will be placed only where desired.
    // Remove 'repr(C)': let Rust optimize.
    remove_attributes(attrs, &["derive", "repr"]);
}

fn transform_ffi(
    rts_src_dir: &Path,
    Context {
        reexports,
        test_items,
        ..
    }: Context,
) -> Result<()> {
    for (relpath, imports) in reexports {
        let path = rts_src_dir.join(relpath);
        let mut syn_file = parse_syn_file(path.as_path())?;
        let mut items = Vec::with_capacity(syn_file.items.len());

        for (module_path, idents) in imports {
            items.push(syn::Item::Use(module_path.to_item_use(idents)));
        }

        for item in syn_file.items {
            match item {
                Item::Const(item_const) => todo!(),
                Item::Enum(item_enum) => todo!(),
                Item::Fn(item_fn) => todo!(),
                Item::Impl(item_impl) => todo!(),
                Item::Macro(item_macro) => todo!(),
                Item::Mod(item_mod) => todo!(),
                Item::Static(item_static) => todo!(),
                Item::Struct(item_struct) => todo!(),
                Item::Trait(item_trait) => todo!(),
                Item::TraitAlias(item_trait_alias) => todo!(),
                Item::Type(item_type) => todo!(),
                Item::Union(item_union) => todo!(),
                Item::Use(item_use) => todo!(),
                item => items.push(item),
            }
        }
        syn_file.items = items;
        fs::write(path.as_path(), format(syn_file).as_bytes())?;

        let test_path: PathBuf = {
            let mut test_path = path.with_extension("");
            test_path.push("tests.rs");
            test_path
        };
        drop(path);

        if !test_path.exists() {
            continue;
        }
        let mut syn_file = parse_syn_file(test_path.as_path())?;
        let mut items = Vec::with_capacity(syn_file.items.len());

        for item in syn_file.items {
            if item_ident(&item).is_none_or(|ident| test_items.contains_key(&ident)) {
                items.push(item);
            }
        }
        syn_file.items = items;
        fs::write(test_path.as_path(), format(syn_file).as_bytes())?;
    }

    Ok(())
}

fn parse_syn_file(path: &Path) -> Result<syn::File> {
    Ok(syn::parse_file(&fs::read_to_string(path)?)?)
}

fn for_each_rs_file<F>(dir: &Path, f: &mut F) -> Result<()>
where
    F: FnMut(&Path) -> Result<()>,
{
    for entry in fs::read_dir(dir)? {
        let path = &*(entry?.path());

        if path.is_dir() {
            for_each_rs_file(path, f)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            f(path)?;
        }
    }
    Ok(())
}

pub fn foreign_item_ident(item: &syn::ForeignItem) -> Option<Ident> {
    match item {
        syn::ForeignItem::Fn(foreign_item_fn) => Some(foreign_item_fn.sig.ident.clone()),
        syn::ForeignItem::Static(foreign_item_static) => Some(foreign_item_static.ident.clone()),
        syn::ForeignItem::Type(foreign_item_type) => Some(foreign_item_type.ident.clone()),
        _ => None,
    }
}
