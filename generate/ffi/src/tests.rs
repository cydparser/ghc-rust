use crate::{Symbols, prefix_with_sys};
use quote::{format_ident, quote};
use std::iter;
use syn::{Ident, Type, parse_quote};

pub fn test_layout(symbols: &Symbols, ident: &Ident) -> syn::ItemFn {
    test_layout_of(
        symbols,
        ident,
        &Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: iter::once(syn::PathSegment {
                    ident: ident.clone(),
                    arguments: syn::PathArguments::default(),
                })
                .collect(),
            },
        }),
    )
}

fn test_layout_of(symbols: &Symbols, ident: &Ident, ty: &Type) -> syn::ItemFn {
    let fn_ident = format_ident!("sys_{}_layout", ident);

    let asserts = {
        let sys_ty = prefix_with_sys(symbols, ty);

        let block: syn::Block = parse_quote! {
            {
                assert_eq!(size_of::<#ty>(), size_of::<#sys_ty>());
                assert_eq!(align_of::<#ty>(), align_of::<#sys_ty>());
            }
        };

        block.stmts
    };

    parse_quote! {
        #[cfg(feature = "sys")]
        #[test]
        fn #fn_ident() {
            #(#asserts)*
        }
    }
}

pub fn test_layout_of_val(ident: &Ident, static_mut: bool) -> syn::ItemFn {
    let fn_ident = format_ident!("sys_{}_layout", ident);

    let attr: Option<syn::Attribute> =
        static_mut.then(|| parse_quote! { #[expect(static_mut_refs)] });

    let asserts = {
        let val = if static_mut {
            quote!(unsafe { &#ident })
        } else {
            quote!(&#ident)
        };

        let sys_val = if static_mut {
            quote!(unsafe { &sys::#ident })
        } else {
            quote!(&sys::#ident)
        };

        let block: syn::Block = parse_quote! {
            {
                assert_eq!(size_of_val(#val), size_of_val(#sys_val));
                assert_eq!(align_of_val(#val), align_of_val(#sys_val));
            }
        };

        block.stmts
    };

    parse_quote! {
        #[cfg(feature = "sys")]
        #[test]
        #attr
        fn #fn_ident() {
            #(#asserts)*
        }
    }
}
