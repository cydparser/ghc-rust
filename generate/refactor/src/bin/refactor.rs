use std::{fs, iter, mem};

use proc_macro2::Span;
use syn::visit_mut::{self, VisitMut};
use syn::{Expr, Path, PathSegment, TypePath};
use syn::{Ident, punctuated::Punctuated};

use generate_refactor::{args_rs, format, has_ffi_attr};

fn main() {
    for path in args_rs().unwrap() {
        eprintln!("  * Refactoring {}", path.display());
        let mut syn_file = syn::parse_file(&fs::read_to_string(&path).unwrap()).unwrap();
        let mut visitor = Refactor::new();

        for item in syn_file.items.iter_mut() {
            visit_mut::visit_item_mut(&mut visitor, item);
        }
        fs::write(path, format(syn_file).as_bytes()).unwrap();
    }
}

pub struct Refactor {
    is_ffi: bool,
}

impl Refactor {
    fn new() -> Refactor {
        Refactor { is_ffi: false }
    }

    fn ffi_scope<F>(&mut self, is_ffi: bool, mut f: F)
    where
        F: FnMut(&mut Refactor),
    {
        let init_is_ffi = self.is_ffi;
        self.is_ffi = is_ffi;
        f(self);
        self.is_ffi = init_is_ffi;
    }
}

impl VisitMut for Refactor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Some(replace) = match expr {
            Expr::Call(expr_call) => replace_atomic_operations(expr_call),
            _ => None,
        } {
            *expr = replace;
        }

        visit_mut::visit_expr_mut(self, expr);
    }

    fn visit_item_mut(&mut self, item: &mut syn::Item) {
        let is_ffi = {
            let attrs = match item {
                syn::Item::Const(i) => Some(&i.attrs),
                syn::Item::Enum(i) => Some(&i.attrs),
                syn::Item::Fn(i) => Some(&i.attrs),
                syn::Item::Static(i) => Some(&i.attrs),
                syn::Item::Struct(i) => Some(&i.attrs),
                syn::Item::Type(i) => Some(&i.attrs),
                syn::Item::Union(i) => Some(&i.attrs),
                _ => None,
            };

            attrs.is_some_and(|attrs| has_ffi_attr(attrs.as_slice()))
        };

        self.ffi_scope(is_ffi, |refactor| {
            visit_mut::visit_item_mut(refactor, item);
        });
    }

    fn visit_type_bare_fn_mut(&mut self, type_bare_fn: &mut syn::TypeBareFn) {
        let is_ffi = type_bare_fn
            .abi
            .as_ref()
            .is_some_and(|abi| abi.name.as_ref().is_some_and(|name| name.value() == "C"));

        self.ffi_scope(is_ffi, |refactor| {
            visit_mut::visit_type_bare_fn_mut(refactor, type_bare_fn);
        });
    }

    fn visit_type_path_mut(&mut self, type_path: &mut TypePath) {
        replace_c_types(self.is_ffi, type_path);

        visit_mut::visit_type_path_mut(self, type_path);
    }
}

/// Replaces old intrinsic atomic operations with newer atomic types.
///
/// E.g. this function will replace ::core::intrinsics::atomic_store_relaxed
/// with Symbol’s function definition is void: &raw.
fn replace_atomic_operations(expr_call: &mut syn::ExprCall) -> Option<Expr> {
    let (name, mut args) = match expr_call.func.as_ref() {
        Expr::Path(syn::ExprPath { path, .. })
            if path.leading_colon.is_some() && path.segments.len() == 3 =>
        {
            let mut segments = path.segments.iter();

            segments.next().filter(|ps| ps.ident == "core")?;
            segments.next().filter(|ps| ps.ident == "intrinsics")?;
            segments.next().and_then(|ps| {
                let name = ps.ident.to_string();

                name.starts_with("atomic_").then(|| {
                    (
                        name,
                        mem::take(&mut expr_call.args)
                            .into_pairs()
                            .map(syn::punctuated::Pair::into_value),
                    )
                })
            })
        }
        _ => None,
    }?;

    fn ordering_expr(order: &str) -> Expr {
        let order = match order {
            "relaxed" => "Relaxed",
            "release" => "Release",
            "acquire" => "Acquire",
            "acqrel" => "AcqRel",
            "seqcst" => "SeqCst",
            _ => panic!("unknown odering: {order}"),
        };

        Expr::Path(syn::ExprPath {
            attrs: vec![],
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: ["Ordering", order]
                    .into_iter()
                    .map(|s| syn::PathSegment {
                        ident: Ident::new(s, Span::call_site()),
                        arguments: syn::PathArguments::None,
                    })
                    .collect(),
            },
        })
    }

    let receiver = Box::new({
        let expr = args.next().unwrap_or_else(|| panic!("empty args: {name}"));

        syn::Expr::Paren(syn::ExprParen {
            attrs: vec![],
            paren_token: Default::default(),
            expr: Box::new(expr),
        })
    });
    let mut method_args: Punctuated<_, _> = args.collect();

    let split_name = name.split('_').collect::<Vec<_>>();

    let method = match *split_name.as_slice() {
        [_, method, order] => {
            method_args.push(ordering_expr(order));

            method
        }
        [_, method, order1, order2] => {
            method_args.push(ordering_expr(order1));
            method_args.push(ordering_expr(order2));

            method
        }
        _ => panic!("unexpected atomic function: {name} {expr_call:?}"),
    };

    Some(Expr::MethodCall(syn::ExprMethodCall {
        attrs: vec![],
        receiver,
        dot_token: Default::default(),
        method: Ident::new(method, Span::call_site()),
        turbofish: None,
        paren_token: Default::default(),
        args: method_args,
    }))
}

// Replace C types in non-FFI items with idiomatic Rust types.
fn replace_c_types(is_ffi: bool, type_path: &mut TypePath) {
    let Some(name) = type_path
        .path
        .get_ident()
        .or_else(|| {
            type_path.path.leading_colon.and_then(|_| {
                let mut segments = type_path.path.segments.iter();
                segments.next().filter(|ps| ps.ident == "libc")?;
                let ps = segments.next()?;
                // Ensure there are no more PathSegments.
                segments.next().is_none().then_some(())?;
                (ps.arguments == syn::PathArguments::None).then_some(&ps.ident)
            })
        })
        .map(|ident| ident.to_string())
    else {
        return;
    };

    enum Ty {
        I8,
        I16,
        I32,
        I64,
        U8,
        U16,
        U32,
        U64,
    }

    use Ty::*;

    let some_type = |ty: Ty| {
        Some(match (ty, is_ffi) {
            (I8, _) => "i8",
            (I16, false) => "i16",
            (I16, true) => "c_short",
            (I32, false) => "i32",
            (I32, true) => "c_int",
            (I64, false) => "i64",
            (I64, true) => "c_long",
            (U8, _) => "u8",
            (U16, false) => "u16",
            (U16, true) => "c_ushort",
            (U32, false) => "u32",
            (U32, true) => "c_uint",
            (U64, false) => "u64",
            (U64, true) => "c_ulong",
        })
    };

    let Some(rename) = (match name.as_str() {
        "__darwin_suseconds_t" => some_type(I32), // NB: libc's suseconds_t is i64.
        "__darwin_time_t" => Some(if is_ffi { "time_t" } else { "i64" }),

        "__int8_t" => some_type(I8),
        "__int16_t" => some_type(I16),
        "__int32_t" => some_type(I32),
        "__int64_t" => some_type(I64),

        "__uint8_t" => some_type(U8),
        "__uint16_t" => some_type(U16),
        "__uint32_t" => some_type(U32),
        "__uint64_t" => some_type(U64),

        "c_double" if !is_ffi => Some("f64"),
        "c_float" if !is_ffi => Some("f32"),

        // c_char is used almost exclusively with C strings.
        // "c_char" => some_type(I8),
        "c_schar" => some_type(I8),
        "c_short" => some_type(I16),
        "c_int" => some_type(I32),
        "c_long" => some_type(I64),
        "c_longlong" => some_type(I64),

        "c_uchar" => some_type(U8),
        "c_ushort" => some_type(U16),
        "c_uint" => some_type(U32),
        "c_ulong" => some_type(U64),
        "c_ulonglong" => some_type(U64),

        "int8_t" => some_type(I8),
        "int32_t" => some_type(I32),
        "int64_t" => some_type(I64),
        "intptr_t" => Some("isize"),

        "size_t" => Some("usize"),
        "ssize_t" => Some("isize"),

        "uint8_t" => some_type(U8),
        "uint16_t" => some_type(U16),
        "uint32_t" => some_type(U32),
        "uint64_t" => some_type(U64),
        "uintptr_t" => Some("usize"),

        "wchar_t" if !is_ffi => Some("char"),
        _ => None,
    }) else {
        return;
    };

    type_path.path = Path {
        leading_colon: None,
        segments: iter::once(PathSegment {
            ident: Ident::new(rename, Span::call_site()),
            arguments: syn::PathArguments::None,
        })
        .collect(),
    };
}
