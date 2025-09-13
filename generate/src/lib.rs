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
    primitive_types: HashSet<Ident>,
    simple_types: HashSet<Ident>,
    pointer_types: HashSet<Ident>,
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
                    hs.insert(Ident::new(s, Span::call_site()));
                }
                hs
            },
            simple_types: {
                let mut hs = HashSet::new();
                for s in symbols::SIMPLE_TYPES {
                    hs.insert(Ident::new(s, Span::call_site()));
                }
                hs
            },
            pointer_types: {
                let mut hs = HashSet::new();
                for s in symbols::POINTER_TYPES {
                    hs.insert(Ident::new(s, Span::call_site()));
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

    pub fn is_primitive_type(&self, ty: &Type) -> bool {
        matches!(ty, Type::Path(tp) if self.is_primitive_type_path(tp))
    }

    pub fn is_primitive_type_path(&self, ty_path: &TypePath) -> bool {
        ty_path
            .path
            .segments
            .last()
            .is_some_and(|ps| self.is_primitive(&ps.ident))
    }

    pub fn is_primitive(&self, ident: &Ident) -> bool {
        self.primitive_types.contains(ident) || Self::looks_primitive(ident)
    }

    fn looks_primitive(ident: &Ident) -> bool {
        ident
            .to_string()
            .chars()
            .next()
            .is_some_and(char::is_lowercase)
            && ident != "c_void"
    }

    /// True for primitives and structs/enums/arrays/slices/tuples containing only "simple" types.
    pub fn is_simple_type(&self, ty: &Type) -> bool {
        match ty {
            Type::Array(type_array) => self.is_simple_type(type_array.elem.as_ref()),
            Type::Path(type_path) => self.is_simple_type_path(type_path),
            Type::Slice(type_slice) => self.is_simple_type(type_slice.elem.as_ref()),
            Type::Tuple(type_tuple) => type_tuple
                .elems
                .iter()
                .all(|param_ty| self.is_simple_type(param_ty)),
            _ => false,
        }
    }

    pub fn is_simple_type_path(&self, type_path: &syn::TypePath) -> bool {
        let Some(ps) = type_path.path.segments.last() else {
            return false;
        };

        let ident = &ps.ident;

        match &ps.arguments {
            syn::PathArguments::None => {
                self.simple_types.contains(ident) || self.is_primitive(ident)
            }
            syn::PathArguments::AngleBracketed(angle_args) => {
                ps.ident == "Option"
                    && matches!(angle_args.args.first(), Some(syn::GenericArgument::Type(param_ty)) if self.is_simple_type(param_ty))
            }
            syn::PathArguments::Parenthesized(_) => unreachable!(),
        }
    }

    pub fn is_simple(&self, ident: &Ident) -> bool {
        self.simple_types.contains(ident) || self.is_primitive(ident)
    }

    pub fn is_pointer_type(&self, ty: &Type) -> bool {
        match ty {
            Type::BareFn(_) => true,
            Type::Path(type_path) => {
                let Some(ident) = type_path.path.get_ident() else {
                    return false;
                };
                self.is_pointer(ident)
            }
            Type::Ptr(_) => true,
            Type::Reference(_) => true,
            _ => false,
        }
    }

    pub fn is_pointer(&self, ident: &Ident) -> bool {
        self.pointer_types.contains(ident)
    }
}
