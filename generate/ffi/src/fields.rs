use crate::Symbols;
use quote::{format_ident, quote};
use syn::{Block, Fields, Ident, Stmt, parse_quote};

pub fn test_layout(symbols: &Symbols, ident: &Ident, fields: &Fields) -> syn::ItemFn {
    let fn_ident = format_ident!("sys_layout_{}", ident);
    let mut asserts: Vec<Stmt> = Vec::with_capacity(fields.len() * 2);

    match fields {
        Fields::Named(fields_named) => {
            for f in &fields_named.named {
                let field = f.ident.as_ref().unwrap();
                let ty = &f.ty;
                let sys_ty = crate::prefix_with_sys(symbols, ty);

                let assert_size = if *ty == sys_ty {
                    None
                } else {
                    Some(quote! {
                        assert_eq!(
                            size_of::<#sys_ty>(),
                            size_of::<#ty>()
                        );
                    })
                };

                let block: Block = parse_quote! {
                    {
                        #assert_size
                        assert_eq!(
                            offset_of!(sys::#ident, #field),
                            offset_of!(#ident, #field),
                        );
                    }
                };

                asserts.extend(block.stmts);
            }
        }
        Fields::Unnamed(_fields_unnamed) => {
            panic!("bindgen produced a tuple struct: {ident}")
        }
        Fields::Unit => (),
    }

    parse_quote! {
        #[cfg(feature = "sys")]
        #[test]
        fn #fn_ident() {
            #(#asserts)*
            assert_eq!(size_of::<sys::#ident>(), size_of::<#ident>());
            assert_eq!(align_of::<sys::#ident>(), align_of::<#ident>());
        }
    }
}
