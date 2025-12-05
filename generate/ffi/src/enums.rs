use proc_macro2 as proc2;
use quote::format_ident;
use syn::{Arm, Expr, ExprLit, Ident, Lit, Variant, parse_quote};

/// Produce an `Iterator` over the Variant's integer discriminants.
pub fn variant_discriminants<'a, I>(variants: I) -> impl Iterator<Item = (isize, &'a Variant)>
where
    I: IntoIterator<Item = &'a Variant>,
{
    let mut next_disc: isize = 0;

    variants.into_iter().map(move |v| {
        let disc = match &v.discriminant {
            Some((
                _,
                Expr::Lit(ExprLit {
                    lit: Lit::Int(int), ..
                }),
            )) => int.base10_parse().unwrap(),
            _ => next_disc,
        };
        next_disc += 1;

        (disc, v)
    })
}

/// Produce TryFrom<u32> for an enum.
pub fn impl_try_from_u32<'a, I>(ident: &Ident, variants: I) -> syn::ItemImpl
where
    I: IntoIterator<Item = &'a Variant>,
{
    let arms: Vec<Arm> = variant_discriminants(variants)
        .map(|(d, v)| {
            let d = Lit::Int(proc2::Literal::isize_unsuffixed(d).into());
            let variant = &v.ident;

            parse_quote! {
                #d => Ok(#variant)
            }
        })
        .collect();

    parse_quote! {
        impl TryFrom<u32> for #ident {
            type Error = ();

            fn try_from(d: u32) -> Result<#ident, ()> {
                use #ident::*;
                match d {
                    #(#arms,)*
                    _ => Err(()),
                }
            }
        }
    }
}

pub fn test_discriminants<'a, I>(ident: &Ident, variants: I) -> syn::ItemFn
where
    I: IntoIterator<Item = &'a Variant>,
{
    let asserts: Vec<Expr> = variants
        .into_iter()
        .map(|v| {
            let variant = &v.ident;

            parse_quote! {
                assert_eq!(sys::#ident::#variant as isize, #ident::#variant as isize)
            }
        })
        .collect();

    let fn_ident = format_ident!("sys_discriminants_{}", ident);

    parse_quote! {
        #[cfg(feature = "sys")]
        #[test]
        fn #fn_ident() {
            #(#asserts);*
        }
    }
}
