use generate_ffi::{self as ffi, Symbols};
use generate_refactor::{UsedIdents, args_rs, format, item_ident};
use proc_macro2::Span;
use quote::format_ident;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::error::Error;
use std::path::{self, Path, PathBuf};
use std::{fs, iter};
use syn::{
    Attribute, ForeignItem, Ident, Item, ItemUse, Token, Type, UseTree, Visibility, parse_quote,
    punctuated::Punctuated, visit,
};

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

        let (syn_file, test_file) = {
            let syn_file = parse_syn_file(&path)?;

            transform(&mut context, false, syn_file)?
        };

        fs::write(&path, format(syn_file).as_bytes())?;

        if let Some(test_file) = test_file {
            let mod_dir = path.with_extension("");
            fs::create_dir_all(&mod_dir)?;
            let test_path = mod_dir.join("tests.rs");

            fs::write(test_path, format(test_file).as_bytes())?;
        }
    }

    transform_ffi(
        &rts_src_dir,
        context.ffi_items,
        context.test_items,
        context.reexports,
    )?;
    create_generated_modules(rts_src_dir.as_path(), context.generated_headers)?;

    Ok(())
}

struct Context {
    file_context: FileContext,
    symbols: Symbols,
    generated_headers: HashMap<Ident, BTreeMap<Ident, Item>>,
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
    used_idents: UsedIdents,
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
            used_idents: UsedIdents::default(),
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
                        ident_modules.insert(ident.clone(), module_path);
                    } else {
                        match item {
                            Item::ForeignMod(item_foreign_mod) => {
                                for fitem in item_foreign_mod.items {
                                    if let Some(ident) = foreign_item_ident(&fitem) {
                                        extern_idents.insert(ident);
                                    }
                                }
                            }
                            Item::Mod(item_mod) => {
                                if let Ok(mod_name) = HeaderModuleName::try_from(&item_mod.ident)
                                    && module_name.should_inline(&mod_name)
                                {
                                    for mitem in
                                        item_mod.content.into_iter().flat_map(|(_, items)| items)
                                    {
                                        if let Some(ident) = item_ident(&mitem) {
                                            ident_modules.insert(ident.clone(), module_path);
                                        }
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
                        test_items.insert(ident.clone(), item);
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
                used_idents: UsedIdents::default(),
            },
            symbols: Symbols::new(),
            header_modules,
            generated_headers: HashMap::new(),
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
            (Item::Const(mut item), FfiItem::Const(ffi, items)) => {
                item.vis = ffi.vis;
                item.attrs = ffi.attrs;
                Ok(iter::once(Item::Const(item)).chain(items).collect())
            }
            (Item::Enum(mut item), FfiItem::Enum(ffi, items)) => {
                item.vis = ffi.vis;
                item.attrs = ffi.attrs;
                Ok(iter::once(Item::Enum(item)).chain(items).collect())
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
            (Item::Struct(mut item), FfiItem::Struct(ffi, items)) => {
                item.vis = ffi.vis;
                item.attrs = ffi.attrs;
                Ok(iter::once(Item::Struct(item)).chain(items).collect())
            }
            (Item::Type(mut item), FfiItem::Type(ffi)) => {
                item.vis = ffi.vis;
                item.attrs = ffi.attrs;
                Ok(vec![Item::Type(item)])
            }
            (Item::Union(mut item), FfiItem::Union(ffi, items)) => {
                item.vis = ffi.vis;
                item.attrs = ffi.attrs;
                Ok(iter::once(Item::Union(item)).chain(items).collect())
            }
            (item, ffi_item) => Err(format!("unable to compute items: {:?}", (item, ffi_item)))?,
        }
    }
}

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct HeaderModuleName(String);

impl TryFrom<&Ident> for HeaderModuleName {
    type Error = &'static str;

    fn try_from(mod_ident: &Ident) -> std::result::Result<Self, Self::Error> {
        HeaderModuleName::try_from(mod_ident.to_string())
    }
}

impl TryFrom<String> for HeaderModuleName {
    type Error = &'static str;

    fn try_from(mut mod_name: String) -> std::result::Result<Self, Self::Error> {
        if mod_name.ends_with("_h") {
            mod_name.make_ascii_lowercase();

            Ok(HeaderModuleName(mod_name))
        } else {
            Err("must end with _h")
        }
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
            (true, "Messages") => mod_name.push_str("rts_Messages_h"),
            (true, "NonMoving") => mod_name.push_str("rts_NonMoving_h"),
            (true, "Ticky") => mod_name.push_str("rts_Ticky_h"),
            (true, "Types") => mod_name.push_str("stg_Types_h"),
            (false, "ForeignExports") => mod_name.push_str("rts_ForeignExports_h"),
            (false, "StableName") => mod_name.push_str("rts_StableName_h"),
            (false, "StablePtr") => mod_name.push_str("rts_StablePtr_h"),
            (false, "Threads") => mod_name.push_str("rts_Threads_h"),
            (false, "Timer") => mod_name.push_str("rts_Timer_h"),
            (false, "GC") => mod_name.push_str("sm_GC_h"),
            _ => {
                mod_name.push_str(name.as_ref());
                mod_name.push_str("_h");
            }
        };
        mod_name.make_ascii_lowercase();

        Ok(HeaderModuleName(mod_name))
    }
}

impl HeaderModuleName {
    fn should_inline(&self, other: &HeaderModuleName) -> bool {
        *self.0 == other.0 || {
            other.0.contains("internal") && {
                let prefix = self.0.strip_suffix("_h").unwrap();

                other
                    .0
                    .strip_prefix(prefix)
                    .is_some_and(|suffix| suffix == "internal_h" || suffix == "internals_h")
            }
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

impl Transformed {
    fn add_items<I>(&mut self, context: &mut Context, items: I)
    where
        I: IntoIterator<Item = syn::Item>,
    {
        for item in items {
            self.add_item(context, item);
        }
    }

    fn add_item(&mut self, context: &mut Context, item: Item) {
        // This is brittle, but the reorganize binary will only run once.
        let possible_test_idents: Vec<Ident> = match &item {
            Item::Const(item_const) => vec![format_ident!("sys_{}_eq", item_const.ident)],
            Item::Enum(item_enum) => {
                let ident = &item_enum.ident;
                vec![
                    format_ident!("sys_{}_discriminants", ident),
                    format_ident!("sys_{}_layout", ident),
                ]
            }
            Item::Fn(item_fn) => {
                let ident = &item_fn.sig.ident;
                vec![
                    format_ident!("test_{}", ident),
                    format_ident!("equivalent_{}", ident),
                ]
            }
            Item::Static(item_static) => {
                vec![format_ident!("sys_{}_layout", item_static.ident)]
            }
            Item::Struct(item_struct) => {
                vec![format_ident!("sys_{}_layout", item_struct.ident)]
            }
            Item::Type(item_type) => vec![format_ident!("sys_{}_layout", item_type.ident)],
            Item::Union(item_union) => {
                vec![format_ident!("sys_{}_layout", item_union.ident)]
            }
            _ => vec![],
        };

        for test_ident in possible_test_idents {
            if let Some(test_item) = context.test_items.remove(&test_ident) {
                self.test_items.push(test_item);
            }
        }

        visit::visit_item(&mut context.file_context.used_idents, &item);

        self.items.push(item)
    }
}

fn transform(
    context: &mut Context,
    in_header: bool,
    syn_file: syn::File,
) -> Result<(syn::File, Option<syn::File>)> {
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

    let used_idents = &context.file_context.used_idents;

    let extern_imports: Vec<syn::ItemUse> = context
        .file_context
        .extern_imports
        .drain()
        .filter_map(|(module_path, mut idents)| {
            idents.retain(|ident| used_idents.contains(ident));

            (!idents.is_empty()).then(|| module_path.to_item_use(idents))
        })
        .collect();

    let test_file = (!transformed.test_items.is_empty()).then(|| syn::File {
        shebang: None,
        attrs: vec![],
        items: [Item::Use(parse_quote! { use super::*; })]
            .into_iter()
            .chain(transformed.test_items)
            .collect(),
    });

    let tests_mod = test_file.is_some().then(|| {
        Item::Mod(parse_quote! {
            #[cfg(test)]
            mod tests;
        })
    });

    let imports = transformed
        .imports
        .into_iter()
        .filter_map(|item_use| used_idents.filter_used(item_use))
        .chain(extern_imports);

    let file = syn::File {
        shebang: None,
        attrs: vec![],
        items: imports
            .map(Item::Use)
            .chain(tests_mod)
            .chain(transformed.items)
            .collect(),
    };

    Ok((file, test_file))
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
        transformed.add_items(context, ffi_item.into_items(Item::Const(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.ident);
        remove_data_attributes(&mut item.attrs);
        transformed.add_item(context, Item::Const(item));
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
        transformed.add_items(context, ffi_item.into_items(Item::Enum(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.ident);

        for variant in item.variants.iter_mut() {
            set_field_visibility(false, in_header, &mut variant.fields);
        }
        remove_data_attributes(&mut item.attrs);
        transformed.add_item(context, Item::Enum(item));
    }

    Ok(())
}

fn transform_fn(
    context: &mut Context,
    in_header: bool,
    transformed: &mut Transformed,
    mut item: syn::ItemFn,
) -> Result<()> {
    let ident = &item.sig.ident;

    if let Some((key, ffi_item)) = context.ffi_items.remove(ident) {
        context.add_reexport(key, ident.clone());
        transformed.add_items(context, ffi_item.into_items(Item::Fn(item))?);
    } else {
        remove_attributes(&mut item.attrs, &[]);
        let consumers = context.symbols.consumers(ident);

        if consumers.is_empty() {
            item.vis = context.visibility(in_header, ident);
            item.sig.abi = None;
        } else {
            // Some Windows-only functions need to be exposed and tested.
            item.vis = Visibility::Public(Default::default());
            item.sig.abi = Some(syn::Abi {
                extern_token: Default::default(),
                name: Some(syn::LitStr::new("C", Span::call_site())),
            });
            let mut attrs = ffi::export_attrs(consumers);
            attrs.extend(item.attrs);
            attrs.push(parse_quote! { #[instrument] });
            item.attrs = attrs;

            if let Some(tests) = ffi::generate_tests(&context.symbols, &item.sig) {
                transformed
                    .test_items
                    .extend(tests.into_iter().map(Item::Fn));
            }
        }
        transformed.add_item(context, Item::Fn(item));
    }

    Ok(())
}

fn transform_foreign_mod(
    context: &mut Context,
    transformed: &mut Transformed,
    mut item_foreign_mod: syn::ItemForeignMod,
) -> Result<()> {
    fn update_extern_imports(context: &mut Context, ident: &Ident) {
        if let Some(&module_path) = context.extern_ident_modules.get(ident) {
            context
                .file_context
                .extern_imports
                .entry(module_path)
                .or_default()
                .push(ident.clone());
        }
    }

    item_foreign_mod.items.retain_mut(|fitem| {
        match fitem {
            ForeignItem::Fn(i) => {
                update_extern_imports(context, &i.sig.ident);
                internal_api(&mut i.vis, &mut i.attrs);
            }
            ForeignItem::Static(i) => {
                update_extern_imports(context, &i.ident);
                internal_api(&mut i.vis, &mut i.attrs);
            }
            ForeignItem::Type(i) => {
                update_extern_imports(context, &i.ident);
                internal_api(&mut i.vis, &mut i.attrs);
            }
            _ => return false,
        }

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
        transformed.add_item(context, Item::ForeignMod(item_foreign_mod));
    }

    Ok(())
}

fn transform_mod(
    context: &mut Context,
    transformed: &mut Transformed,
    mut item_mod: syn::ItemMod,
) -> Result<()> {
    let mod_ident = &item_mod.ident;
    let mod_name = HeaderModuleName::try_from(mod_ident);

    if mod_name
        .iter()
        .any(|mod_name| context.file_context.module_name.should_inline(mod_name))
    {
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
                item_mod.vis = Visibility::Inherited;

                if let Some((_, items)) = item_mod.content.take() {
                    let generated_items = context
                        .generated_headers
                        .entry(mod_ident.clone())
                        .or_default();

                    for mut item in items {
                        let ident = match &mut item {
                            Item::Const(i) => {
                                internal_api(&mut i.vis, &mut i.attrs);
                                &i.ident
                            }
                            Item::Enum(i) => {
                                internal_api(&mut i.vis, &mut i.attrs);
                                &i.ident
                            }
                            Item::Fn(i) => {
                                i.sig.abi = None;
                                internal_api(&mut i.vis, &mut i.attrs);
                                &i.sig.ident
                            }
                            Item::Static(i) => {
                                internal_api(&mut i.vis, &mut i.attrs);
                                &i.ident
                            }
                            Item::Struct(i) => {
                                internal_api(&mut i.vis, &mut i.attrs);
                                &i.ident
                            }
                            Item::Type(i) => {
                                internal_api(&mut i.vis, &mut i.attrs);
                                &i.ident
                            }
                            Item::Union(i) => {
                                internal_api(&mut i.vis, &mut i.attrs);
                                &i.ident
                            }
                            _ => continue,
                        };

                        generated_items.insert(ident.clone(), item);
                    }
                }

                return Ok(());
            }

            if let Some(relpath) = relpath.strip_prefix("rts/") {
                let (is_ffi, relpath) = match relpath.strip_prefix("include/") {
                    Some(relpath) => (true, relpath),
                    None => (false, relpath),
                };

                let mut module_path = ModulePath::from_filepath(relpath);

                if is_ffi {
                    module_path.0.insert(0, "ffi".to_string());
                }

                if let Ok(mod_name) = mod_name {
                    context.header_modules.insert(mod_name, module_path);
                }
            }
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
        transformed.add_items(context, ffi_item.into_items(Item::Static(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.ident);
        remove_attributes(&mut item.attrs, &[]);
        transformed.add_item(context, Item::Static(item));
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
        transformed.add_items(context, ffi_item.into_items(Item::Struct(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.ident);
        set_field_visibility(false, in_header, &mut item.fields);
        remove_data_attributes(&mut item.attrs);
        transformed.add_item(context, Item::Struct(item));
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
        transformed.add_items(context, ffi_item.into_items(Item::Type(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.ident);
        remove_data_attributes(&mut item.attrs);
        transformed.add_item(context, Item::Type(item));
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
        transformed.add_items(context, ffi_item.into_items(Item::Union(item))?);
    } else {
        item.vis = context.visibility(in_header, &item.ident);
        set_named_field_visibility(false, in_header, &mut item.fields);
        remove_data_attributes(&mut item.attrs);
        transformed.add_item(context, Item::Union(item));
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

        if let Some(module_path) = HeaderModuleName::try_from(ident)
            .ok()
            .and_then(|mod_name| context.header_modules.get(&mod_name))
        {
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
        } else if context.generated_headers.contains_key(ident) {
            item.tree = UseTree::Path(syn::UsePath {
                ident: Ident::new("crate", Span::call_site()),
                colon2_token: Default::default(),
                tree: Box::new(UseTree::Path(use_path)),
            });
            item.vis = Visibility::Inherited;
            transformed.add_item(context, syn::Item::Use(item));
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

fn internal_api(vis: &mut Visibility, attrs: &mut Vec<Attribute>) {
    *vis = parse_quote! { pub(crate) };
    remove_attributes(attrs, &[]);
}

fn transform_ffi(
    rts_src_dir: &Path,
    ffi_items: HashMap<Ident, (&'static Path, FfiItem)>,
    test_items: HashMap<Ident, Item>,
    reexports: HashMap<&'static Path, BTreeMap<&'static ModulePath, Vec<Ident>>>,
) -> Result<()> {
    for (relpath, crate_imports) in reexports {
        eprintln!("  * Adjusting FFI {relpath:?}");
        let path = rts_src_dir.join(relpath);

        type OnlyImports = bool;

        fn filter_items<I, F>(items: I, f: F) -> (OnlyImports, Vec<Item>)
        where
            I: IntoIterator<Item = syn::Item>,
            F: Fn(&Item) -> bool,
        {
            let mut used_idents = UsedIdents::default();
            let mut imports = Vec::with_capacity(8);
            let iter = items.into_iter();
            let mut other_items = Vec::with_capacity(iter.size_hint().0);

            for item in iter {
                match item {
                    Item::Use(item_use) => imports.push(item_use),
                    item => {
                        if f(&item) {
                            visit::visit_item(&mut used_idents, &item);
                            other_items.push(item);
                        }
                    }
                }
            }

            (
                other_items.is_empty(),
                imports
                    .into_iter()
                    .filter_map(|item_use| used_idents.filter_used(item_use))
                    .map(Item::Use)
                    .chain(other_items)
                    .collect(),
            )
        }

        let has_tests = {
            let mut test_path: PathBuf = path.with_extension("");
            test_path.push("tests.rs");
            let test_path = &test_path;

            if test_path.exists() {
                let mut syn_file = parse_syn_file(test_path)?;

                let (only_imports, items) = filter_items(syn_file.items, |item| {
                    item_ident(item).is_none_or(|ident| test_items.contains_key(ident))
                });

                if only_imports {
                    fs::remove_file(test_path)?;
                    false
                } else {
                    syn_file.items = items;
                    fs::write(test_path, format(syn_file).as_bytes())?;
                    true
                }
            } else {
                false
            }
        };

        let imports = {
            let mut imports = Vec::with_capacity(crate_imports.len());

            for (module_path, idents) in crate_imports {
                imports.push(syn::Item::Use({
                    let mut item_use = module_path.to_item_use(idents);
                    item_use.vis = Visibility::Public(Default::default());

                    item_use
                }));
            }
            imports
        };

        let mut syn_file = parse_syn_file(path.as_path())?;

        let (_, items) = filter_items(imports.into_iter().chain(syn_file.items), |item| {
            if let Some(ident) = item_ident(item) {
                ffi_items.contains_key(ident)
            } else {
                match &item {
                    Item::ForeignMod(_) => true,
                    Item::Impl(item_impl) => {
                        // Check both A and B in `impl Trait<A> for B`.
                        if let Type::Path(type_path) = &*item_impl.self_ty
                            && let Some(ident) = type_path.path.get_ident()
                            && ffi_items.contains_key(ident)
                        {
                            true
                        } else {
                            if let Some((_, trait_path, _)) = &item_impl.trait_
                                && let Some(ps) = trait_path.segments.last()
                                && let syn::PathArguments::AngleBracketed(args) = &ps.arguments
                                && let Some(ident) = args.args.iter().find_map(|arg| match arg {
                                    syn::GenericArgument::Type(Type::Path(type_path)) => {
                                        type_path.path.get_ident()
                                    }
                                    _ => None,
                                })
                            {
                                ffi_items.contains_key(ident)
                            } else {
                                false
                            }
                        }
                    }
                    Item::Mod(item_mod) => has_tests || item_mod.ident != "tests",
                    _ => panic!("unexpected item variant: {item:?}"),
                }
            }
        });

        syn_file.items = items;
        fs::write(path.as_path(), format(syn_file).as_bytes())?;
    }

    Ok(())
}

fn create_generated_modules(
    dir: &Path,
    generated_headers: HashMap<Ident, BTreeMap<Ident, Item>>,
) -> Result<()> {
    for (mod_ident, generated_items) in generated_headers {
        let path = {
            let mut file_name = mod_ident.to_string();

            if file_name.ends_with("_h") {
                file_name.truncate(file_name.len() - 2);
            }
            let mut file_name = stringcase::snake_case(file_name.as_str());
            file_name.push_str(".rs");

            dir.join(file_name)
        };

        eprintln!("  * Moving generated header to {path:?}");

        let syn_file = syn::File {
            shebang: None,
            attrs: vec![],
            items: generated_items.into_values().collect(),
        };

        fs::write(path.as_path(), format(syn_file).as_bytes())?;
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
