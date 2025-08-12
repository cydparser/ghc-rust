use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;

/// Like [tracing::instrument](https://docs.rs/tracing/latest/tracing/attr.instrument.html), but
/// skips all fields by default.
#[proc_macro_attribute]
pub fn instrument(args: TokenStream, item: TokenStream) -> TokenStream {
    if cfg!(not(feature = "tracing")) {
        return item;
    }

    let attr: TokenStream = {
        let args: TokenStream2 = if args.is_empty() {
            TokenTree::Ident(Ident::new("skip_all", Span::call_site())).into()
        } else {
            args.into()
        };
        quote!(#[::tracing::instrument(#args)]).into()
    };

    let mut ts = TokenStream::new();
    ts.extend(attr);
    ts.extend(item);
    ts
}
