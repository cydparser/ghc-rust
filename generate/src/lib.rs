use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use proc_macro2::Span;
use syn::{Ident, Type, TypePath};

mod place;
mod symbols;

pub use crate::place::PlacesIter;
pub use crate::symbols::{Place, Places};

pub struct Symbols {
    internal_module: bool,
    internal_modules: HashSet<PathBuf>,
    symbols: HashMap<Ident, Places>,
    primitive_types: HashSet<TypePath>,
    simple_types: HashSet<Ident>,
    pointer_types: HashSet<TypePath>,
}

impl Symbols {
    #[expect(clippy::new_without_default)]
    pub fn new() -> Symbols {
        Symbols {
            internal_module: false,
            internal_modules: {
                let mut hs = HashSet::new();
                #[expect(clippy::single_element_loop)]
                for s in ["rts/capability.rs"] {
                    hs.insert(PathBuf::from(s));
                }
                hs
            },
            symbols: {
                let mut hs = HashMap::new();
                for (sym, places) in symbols::SYMBOLS {
                    hs.insert(Ident::new(sym, Span::call_site()), places);
                }
                hs
            },
            primitive_types: {
                let mut hs = HashSet::new();
                for s in symbols::PRIMITIVE_TYPES {
                    hs.insert(type_path(s));
                }
                hs
            },
            simple_types: HashSet::new(),
            pointer_types: {
                let mut hs = HashSet::new();
                for s in symbols::POINTER_TYPES {
                    hs.insert(type_path(s));
                }
                hs
            },
        }
    }

    pub fn with_module(&mut self, path: &Path) {
        self.internal_module = self.internal_modules.contains(path);
    }

    pub fn insert_simple_type(&mut self, ident: Ident) {
        self.simple_types.insert(ident);
    }

    pub fn is_internal_module(&self) -> bool {
        self.internal_module
    }

    pub fn places(&self, ident: &Ident) -> Places {
        self.symbols.get(ident).copied().unwrap_or_default()
    }

    pub fn is_primitive_type(&self, ty_path: &TypePath) -> bool {
        self.primitive_types.contains(ty_path)
            || ty_path.path.segments.last().is_some_and(|p| {
                p.ident
                    .to_string()
                    .chars()
                    .next()
                    .is_some_and(char::is_lowercase)
            })
    }

    /// True for primitives and structs/enums/arrays/slices/tuples containing only "simple" types.
    pub fn is_simple_type(&self, ty: &Type) -> bool {
        match ty {
            Type::Array(type_array) => self.is_simple_type(type_array.elem.as_ref()),
            Type::Path(type_path) => match type_path.path.get_ident() {
                Some(ident) if self.simple_types.contains(ident) => true,
                _ => self.is_primitive_type(type_path),
            },
            Type::Slice(type_slice) => self.is_simple_type(type_slice.elem.as_ref()),
            Type::Tuple(type_tuple) => type_tuple
                .elems
                .iter()
                .all(|param_ty| self.is_simple_type(param_ty)),
            _ => false,
        }
    }

    pub fn is_pointer_type(&self, ty: &Type) -> bool {
        match ty {
            Type::BareFn(_) => true,
            Type::Path(type_path) => self.pointer_types.contains(type_path),
            Type::Ptr(_) => true,
            Type::Reference(_) => true,
            _ => false,
        }
    }
}

fn type_path(ident: &str) -> TypePath {
    syn::TypePath {
        qself: None,
        path: syn::Path {
            leading_colon: None,
            segments: [syn::PathSegment {
                ident: Ident::new(ident, Span::call_site()),
                arguments: syn::PathArguments::None,
            }]
            .into_iter()
            .collect(),
        },
    }
}
