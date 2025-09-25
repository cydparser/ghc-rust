use crate::Symbols;
use proc_macro2::{Punct, Spacing, TokenStream, TokenTree};
use quote::{format_ident, quote};
use std::iter;
use syn::Token;
use syn::{Ident, Type, parse_quote, punctuated::Punctuated, token};

pub fn generate_tests(
    symbols: &Symbols,
    sig: &syn::Signature,
) -> Option<impl IntoIterator<Item = syn::ItemFn>> {
    let (mut has_arbitrary, mut has_todo) = (false, false);

    let mut check_is_simple = |is_simple: bool| {
        if is_simple {
            has_arbitrary = true;
        } else {
            has_todo = true;
        }
    };

    let mut equiv_params: Punctuated<syn::FnArg, token::Comma> = Default::default();
    let mut args: Punctuated<syn::Expr, token::Comma> = Default::default();
    let (mut equiv_expect_lets, mut equiv_actual_lets) = (vec![], vec![]);
    let mut unit_lets = vec![];

    for Arg { ident, ty_details } in as_args(symbols, &sig.inputs)? {
        let TypeDetails {
            is_simple,
            indirection_levels,
            ty,
        } = ty_details;

        check_is_simple(is_simple);

        let (equiv_mut, unit_mut): (Option<Token![mut]>, Option<Token![mut]>) =
            match indirection_levels {
                0 => (None, None),
                1 => (Some(Default::default()), None),
                2.. => (Some(Default::default()), Some(Default::default())),
            };

        if is_simple {
            equiv_params.push(parse_quote! { #ident: #ty });

            let into = if symbols.is_primitive_type(ty) {
                None
            } else {
                Some(quote! { .into() })
            };

            equiv_expect_lets.push(quote! { let #equiv_mut #ident = #ident.clone()#into });
            equiv_actual_lets.push(quote! { let #equiv_mut #ident = #ident.clone() });
            unit_lets.push(quote! { let #equiv_mut #ident: #ty = Arbitrary::arbitrary(g) });
        } else {
            let let_todo = quote! { let #equiv_mut #ident: #ty = todo!() };
            equiv_expect_lets.push(let_todo.clone());
            equiv_actual_lets.push(let_todo.clone());
            unit_lets.push(quote! { let #unit_mut #ident: #ty = todo!() });
        }

        if indirection_levels > 1 {
            let lets: Vec<_> = iter::repeat_n(
                quote! { let mut #ident = &raw mut #ident },
                indirection_levels - 1,
            )
            .collect();

            equiv_expect_lets.extend(lets.clone());
            equiv_actual_lets.extend(lets.clone());
            unit_lets.extend(lets);
        }

        args.push(if indirection_levels == 0 {
            parse_quote! { #ident }
        } else {
            parse_quote! { &raw mut #ident }
        });
    }

    let (cmp_ty, deref, return_is_simple) = match &sig.output {
        syn::ReturnType::Default => (None, None, true),
        syn::ReturnType::Type(_, ret_ty) => {
            let TypeDetails {
                is_simple,
                indirection_levels,
                ty,
            } = as_type_details(symbols, ret_ty.as_ref())?;

            let cmp_ty;
            let deref;

            if indirection_levels > 0 {
                cmp_ty = Some(Type::Reference(syn::TypeReference {
                    and_token: Default::default(),
                    lifetime: None,
                    mutability: None,
                    elem: Box::new(ty.clone()),
                }));
                deref = {
                    let mut ts = TokenStream::new();
                    ts.extend([TokenTree::Punct(Punct::new('&', Spacing::Alone))]);
                    ts.extend(
                        iter::repeat_n(
                            TokenTree::Punct(Punct::new('*', Spacing::Alone)),
                            indirection_levels,
                        )
                        .collect::<Vec<_>>(),
                    );
                    Some(ts)
                };
            } else {
                cmp_ty = Some(ty.clone());
                deref = None;
            }

            (cmp_ty, deref, is_simple)
        }
    };

    let ident = &sig.ident;

    let call_sys = quote! { unsafe { #deref sys::#ident(#args) } };
    let call = quote! { unsafe { #deref #ident(#args) } };

    Some(vec![
        generate_equivalent_test(
            ident,
            equiv_params,
            &cmp_ty,
            (equiv_expect_lets, equiv_actual_lets),
            has_todo,
            &call_sys,
            &call,
        ),
        generate_unit_test(
            ident,
            (&cmp_ty, return_is_simple),
            unit_lets,
            has_arbitrary,
            &call,
        ),
    ])
}

struct Arg<'a> {
    ident: &'a Ident,
    ty_details: TypeDetails<'a>,
}

struct TypeDetails<'a> {
    is_simple: bool,
    indirection_levels: usize,
    ty: &'a syn::Type,
}

fn as_args<'a, 'b, Args: IntoIterator<Item = &'a syn::FnArg>>(
    symbols: &'b Symbols,
    input: Args,
) -> Option<Vec<Arg<'a>>> {
    let input = input.into_iter();
    let mut args = Vec::with_capacity(input.size_hint().0);

    for arg in input {
        let syn::FnArg::Typed(pat_type) = arg else {
            panic!("Unexpected FnArg::Recever: {arg:#?}")
        };
        let syn::Pat::Ident(syn::PatIdent {
            by_ref: None,
            mutability: None,
            ident,
            subpat: None,
            ..
        }) = pat_type.pat.as_ref()
        else {
            panic!("Expected only syn::Pat::Ident: {arg:#?}");
        };
        args.push(Arg {
            ident,
            ty_details: as_type_details(symbols, pat_type.ty.as_ref())?,
        });
    }

    Some(args)
}

fn as_type_details<'a>(symbols: &Symbols, ty: &'a syn::Type) -> Option<TypeDetails<'a>> {
    fn indirection_ty_levels(ty: &Type, levels: usize) -> (&Type, usize) {
        match ty {
            syn::Type::Ptr(type_ptr) => indirection_ty_levels(type_ptr.elem.as_ref(), levels + 1),
            _ => (ty, levels),
        }
    }

    match ty {
        syn::Type::Never(_) => None,
        syn::Type::BareFn(_) => Some(TypeDetails {
            is_simple: false,
            indirection_levels: 0,
            ty,
        }),
        syn::Type::Path(type_path) => Some(TypeDetails {
            is_simple: symbols.is_simple_type_path(type_path),
            indirection_levels: 0,
            ty,
        }),
        syn::Type::Ptr(type_ptr) => {
            let (inner_ty, indirection_levels) = indirection_ty_levels(type_ptr.elem.as_ref(), 1);
            Some(TypeDetails {
                is_simple: symbols.is_simple_type(inner_ty),
                indirection_levels,
                ty: inner_ty,
            })
        }
        _ => panic!("unexpected Type: {ty:?}"),
    }
}

fn generate_equivalent_test(
    ident: &Ident,
    params: Punctuated<syn::FnArg, token::Comma>,
    cmp_ty: &Option<Type>,
    (expect_lets, actual_lets): (Vec<TokenStream>, Vec<TokenStream>),
    mut has_todo: bool,
    call_sys: &TokenStream,
    call: &TokenStream,
) -> syn::ItemFn {
    let (test, ret) = if !params.is_empty() {
        (quote! { #[quickcheck] }, Some(quote! { -> bool }))
    } else {
        (quote! { #[test] }, None)
    };

    let equiv_fn = format_ident!("equivalent_{}", ident);

    let (expected, actual) = {
        let ty_sig = cmp_ty.as_ref().map(|ty| quote! { : #ty });

        let todo = if cmp_ty.is_none() {
            has_todo = true;
            Some(quote! { ; todo!() })
        } else {
            None
        };

        (
            quote! {
                let expected #ty_sig = {
                    #(#expect_lets;)*
                    #call_sys
                    #todo
                };
            },
            quote! {
                let actual #ty_sig = {
                    #(#actual_lets;)*
                    #call
                    #todo
                };
            },
        )
    };

    let check = if params.is_empty() {
        quote! { assert_eq!(expected, actual); }
    } else {
        quote! { expected == actual }
    };

    let attrs = if has_todo {
        Some(quote! { #[expect(unreachable_code, unused_variables)] })
    } else {
        None
    };

    parse_quote! {
        #test
        #[ignore]
        #attrs
        fn #equiv_fn(#params) #ret {
            #expected
            #actual
            #check
        }
    }
}

fn generate_unit_test(
    ident: &Ident,
    (cmp_ty, return_is_simple): (&Option<Type>, bool),
    lets: Vec<TokenStream>,
    has_arbitrary: bool,
    call: &TokenStream,
) -> syn::ItemFn {
    let unit_fn = format_ident!("test_{}", ident);

    let let_g = if has_arbitrary {
        Some(quote! { let g = &mut Gen::new(100); })
    } else {
        None
    };

    let (expected, actual) = {
        let ty_sig = cmp_ty.as_ref().map(|ty| quote! { : #ty });

        let let_result = if return_is_simple {
            None
        } else {
            Some(quote! { let result: #cmp_ty = })
        };

        let todo = if cmp_ty.is_none() || let_result.is_some() {
            Some(quote! { ; todo!() })
        } else {
            None
        };

        (
            quote! { let expected #ty_sig = todo!(); },
            quote! {
                let actual #ty_sig = {
                    #(#lets;)*
                    #let_result #call
                    #todo
                };
            },
        )
    };

    parse_quote! {
        #[test]
        #[ignore]
        #[expect(unreachable_code, unused_variables)]
        fn #unit_fn() {
            #let_g
            #actual
            #expected
            assert_eq!(expected, actual);
        }
    }
}
