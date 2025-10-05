use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;

mod ffi;

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

/// cbindgen does not expand macros with non-nightly toolchains, and so does not see no_mangle when
/// wrapped in cfg_attr (see https://github.com/mozilla/cbindgen/issues/183). To get around this, we
/// add cfg_attr and export_name with this macro.
#[proc_macro_attribute]
pub fn ffi(args: TokenStream, item: TokenStream) -> TokenStream {
    match ffi::ffi_attribute(args.into(), item.into()) {
        Ok(item) => item.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
