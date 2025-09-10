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

    pub fn is_pointer_type(&self, ty: &Type) -> bool {
        match ty {
            Type::Path(type_path) => self.pointer_types.contains(type_path),
            Type::Ptr(_) => true,
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
