use std::path::PathBuf;
use syn::{Ident, Item};

mod used_idents;
pub use used_idents::*;

pub fn args_rs() -> Result<Vec<PathBuf>, String> {
    let (rs, non_rs): (Vec<_>, Vec<_>) = std::env::args().partition(|arg| arg.ends_with(".rs"));

    match non_rs.as_slice() {
        // Skip the first, program name arg.
        [] | [_] => Ok(rs.into_iter().map(PathBuf::from).collect()),
        _ => Err(format!(
            "expected files ending in .rs; saw {:?}",
            &non_rs[1..]
        )),
    }
}

pub fn item_ident(item: &Item) -> Option<&Ident> {
    match item {
        Item::Const(item_const) => Some(&item_const.ident),
        Item::Enum(item_enum) => Some(&item_enum.ident),
        Item::Fn(item_fn) => Some(&item_fn.sig.ident),
        Item::Static(item_static) => Some(&item_static.ident),
        Item::Struct(item_struct) => Some(&item_struct.ident),
        Item::Type(item_type) => Some(&item_type.ident),
        Item::Union(item_union) => Some(&item_union.ident),
        _ => None,
    }
}

pub fn format(syn_file: syn::File) -> String {
    let pretty = prettyplease::unparse(&syn_file);

    generate_format::add_blank_lines(pretty.as_ref())
}
