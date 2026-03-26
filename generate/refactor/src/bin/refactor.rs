use generate_refactor::{args_rs, format};
use proc_macro2::Span;
use std::{fs, mem};
use syn::Expr;
use syn::visit_mut::{self, VisitMut};
use syn::{Ident, punctuated::Punctuated};

fn main() {
    for path in args_rs().unwrap() {
        eprintln!("  * Refactoring {}", path.display());
        let mut syn_file = syn::parse_file(&fs::read_to_string(&path).unwrap()).unwrap();
        let mut visitor = Refactor;

        for item in syn_file.items.iter_mut() {
            visit_mut::visit_item_mut(&mut visitor, item);
        }
        fs::write(path, format(syn_file).as_bytes()).unwrap();
    }
}

pub struct Refactor;

impl VisitMut for Refactor {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        replace_atomic_operations(expr);

        visit_mut::visit_expr_mut(self, expr);
    }
}

/// Replaces old intrinsic atomic operations with newer atomic types.
///
/// E.g. this function will replace `::core::intrinsics::atomic_store_relaxed(&raw mut x, 0)`
/// with `(&raw mut x).store(0, Ordering::Relaxed)`.
fn replace_atomic_operations(expr: &mut Expr) {
    // This can be replaced after try_block are stable.
    fn atomic_name_args(expr: &mut Expr) -> Option<(String, impl Iterator<Item = Expr>)> {
        match expr {
            Expr::Call(syn::ExprCall { func, args, .. }) => match func.as_ref() {
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
                                mem::take(args)
                                    .into_pairs()
                                    .map(syn::punctuated::Pair::into_value),
                            )
                        })
                    })
                }
                _ => None,
            },
            _ => None,
        }
    }

    let Some((name, mut args)) = atomic_name_args(expr) else {
        return;
    };

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
        _ => panic!("unexpected atomic function: {name} {expr:?}"),
    };

    *expr = Expr::MethodCall(syn::ExprMethodCall {
        attrs: vec![],
        receiver,
        dot_token: Default::default(),
        method: Ident::new(method, Span::call_site()),
        turbofish: None,
        paren_token: Default::default(),
        args: method_args,
    });
}
