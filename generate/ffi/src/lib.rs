use generate_consumers::Consumers;
use generate_symbols as symbols;
use proc_macro2::Span;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use syn::{Ident, Type, TypePath, parse_quote};

mod tests;

pub use crate::tests::generate_tests;

pub struct Symbols {
    internal_module: bool,
    internal_modules: HashSet<PathBuf>,
    symbols: HashMap<Ident, Consumers>,
    primitive_types: HashSet<Ident>,
    copy_types: HashSet<Ident>,
    simple_types: HashSet<Ident>,
    non_simple_types: HashSet<Ident>,
    pointer_types: HashSet<Ident>,
    option_types: HashSet<Ident>,
    std_types: HashSet<Ident>,
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
                for (sym, c) in symbols::SYMBOLS {
                    hs.insert(
                        Ident::new(sym, Span::call_site()),
                        Consumers::try_from(c).unwrap(),
                    );
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
            copy_types: {
                let mut hs = HashSet::new();
                for s in symbols::COPY_TYPES {
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
            non_simple_types: {
                let mut hs = HashSet::new();
                for s in symbols::NON_SIMPLE_TYPES {
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
            option_types: {
                let mut hs = HashSet::new();
                for s in symbols::OPTION_TYPES {
                    hs.insert(Ident::new(s, Span::call_site()));
                }
                hs
            },
            std_types: {
                let mut hs = HashSet::new();
                for s in symbols::STD_TYPES {
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

    pub fn consumers(&self, ident: &Ident) -> Consumers {
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
        self.primitive_types.contains(ident) || self.looks_primitive(ident)
    }

    fn looks_primitive(&self, ident: &Ident) -> bool {
        ident
            .to_string()
            .chars()
            .next()
            .is_some_and(char::is_lowercase)
            && !self.non_simple_types.contains(ident)
    }

    pub fn is_copy_type(&self, ty: &Type) -> bool {
        match ty {
            Type::BareFn(_) => true,
            Type::Path(type_path) => type_path
                .path
                .segments
                .last()
                .is_some_and(|p| self.copy_types.contains(&p.ident)),
            ty => panic!("unexpected type: {ty:?}"),
        }
    }

    pub fn is_copy(&self, ident: &Ident) -> bool {
        self.copy_types.contains(ident)
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

    pub fn is_option_type(&self, ty: &Type) -> bool {
        match ty {
            Type::Path(type_path) => {
                let Some(ident) = type_path.path.get_ident() else {
                    return false;
                };
                self.is_option(ident)
            }
            _ => false,
        }
    }

    pub fn is_option(&self, ident: &Ident) -> bool {
        self.option_types.contains(ident)
    }

    pub fn is_std_type(&self, ty: &Type) -> bool {
        match ty {
            Type::Array(type_array) => self.is_std_type(type_array.elem.as_ref()),
            Type::BareFn(type_bare_fn) => {
                type_bare_fn
                    .inputs
                    .iter()
                    .all(|arg| self.is_std_type(&arg.ty))
                    && match &type_bare_fn.output {
                        syn::ReturnType::Default => true,
                        syn::ReturnType::Type(_, rty) => self.is_std_type(rty.as_ref()),
                    }
            }
            Type::Never(_) => true,
            Type::Path(type_path) => self.is_std_type_path(type_path),
            Type::Ptr(type_ptr) => self.is_std_type(type_ptr.elem.as_ref()),
            Type::Reference(type_ref) => self.is_std_type(type_ref.elem.as_ref()),
            ty => panic!("Unexpected type: {ty:?}"),
        }
    }

    pub fn is_std_type_path(&self, type_path: &syn::TypePath) -> bool {
        match type_path.path.segments.last() {
            None => unreachable!(),
            Some(ps) => {
                let ident = &ps.ident;

                match &ps.arguments {
                    syn::PathArguments::AngleBracketed(angle_args) => {
                        ident == "Option"
                            && matches!(angle_args.args.first(), Some(syn::GenericArgument::Type(param_ty)) if self.is_std_type(param_ty))
                    }
                    _ => self.is_std(ident),
                }
            }
        }
    }

    pub fn is_std(&self, ident: &Ident) -> bool {
        self.std_types.contains(ident) || self.is_primitive(ident) || ident == "c_void"
    }
}

pub fn prefix_with_sys(ty: &Type) -> Type {
    match ty {
        Type::Array(type_array) => {
            let mut array = type_array.clone();
            array.elem = Box::new(prefix_with_sys(type_array.elem.as_ref()));
            Type::Array(array)
        }
        Type::Path(type_path) => parse_quote! { sys::#type_path },
        Type::Ptr(type_ptr) => {
            let mut ptr = type_ptr.clone();
            ptr.elem = Box::new(prefix_with_sys(type_ptr.elem.as_ref()));
            Type::Ptr(ptr)
        }
        Type::Reference(type_reference) => {
            let mut ty_ref = type_reference.clone();
            ty_ref.elem = Box::new(prefix_with_sys(type_reference.elem.as_ref()));
            Type::Reference(ty_ref)
        }
        ty => panic!("Unexpected type: {ty:?}"),
    }
}
