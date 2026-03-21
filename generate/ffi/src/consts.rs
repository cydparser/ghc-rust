use crate::tests::test_layout_of_val;
use quote::format_ident;
use syn::{Ident, parse_quote};

pub fn tests(ident: &Ident) -> [syn::ItemFn; 2] {
    let test_eq = format_ident!("sys_{}_eq", ident);

    [
        parse_quote! {
            #[cfg(feature = "sys")]
            #[test]
            fn #test_eq() {
                assert_eq!(#ident, sys::#ident);
            }
        },
        test_layout_of_val(ident, false),
    ]
}
