use proc_macro2 as proc2;
use proc_macro2::{Delimiter, Group, Spacing, TokenStream, TokenTree};
use syn::{Ident, Token};

pub fn ffi_attribute(_args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let item: FfiItem = syn::parse2(item)?;
    Ok([item.attrs, item.code].into_iter().collect())
}

struct FfiItem {
    attrs: TokenStream,
    code: TokenStream,
}

impl syn::parse::Parse for FfiItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attrs = TokenStream::new();
        let mut code = TokenStream::new();

        while input.peek(Token![#]) {
            let pound: TokenTree = input.parse()?;
            let mut group: Group = input.parse()?;

            let is_no_mangle = {
                let mut group_iter = group.stream().into_iter();

                if let Some(TokenTree::Ident(ident)) = group_iter.next() {
                    if ident == "no_mangle" {
                        true
                    } else if ident == "unsafe"
                        && let Some(TokenTree::Group(unsafe_group)) = group_iter.next()
                        && let Some(TokenTree::Ident(ident)) =
                            unsafe_group.stream().into_iter().next()
                        && ident == "no_mangle"
                    {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };

            if is_no_mangle {
                let span = group.span();
                // Set group to: new_cfg_attr(not(feature = "sys"), unsafe(no_mangle))
                group = new_cfg_attr(
                    span,
                    [
                        TokenTree::Ident(Ident::new("not", span)),
                        TokenTree::Group(new_group([
                            TokenTree::Ident(Ident::new("feature", span)),
                            TokenTree::Punct(proc2::Punct::new('=', Spacing::Alone)),
                            TokenTree::Literal(proc2::Literal::string("sys")),
                        ])),
                    ],
                    group.stream(),
                );
                group.set_span(span);
            }
            attrs.extend([pound, TokenTree::Group(group)]);
        }

        #[derive(PartialEq)]
        enum ItemKind {
            Const,
            Fn,
            Static,
            Struct,
            Type,
            Union,
        }

        let mut kind = None;

        let mut item_ident: Option<Ident> = None;

        while let Ok(tt) = input.parse::<TokenTree>() {
            if let TokenTree::Ident(ident) = &tt {
                if kind.is_none() {
                    kind = if ident == "const" {
                        Some(ItemKind::Const)
                    } else if ident == "fn" {
                        Some(ItemKind::Fn)
                    } else if ident == "static" {
                        Some(ItemKind::Static)
                    } else if ident == "struct" {
                        Some(ItemKind::Struct)
                    } else if ident == "type" {
                        Some(ItemKind::Type)
                    } else if ident == "union" {
                        Some(ItemKind::Union)
                    } else {
                        None
                    };
                } else {
                    item_ident = Some(ident.clone());
                    code.extend([tt]);
                    code.extend(input.parse::<TokenStream>()?);
                    break;
                }
            }
            code.extend([tt]);
        }

        let Some(kind) = kind else {
            return Err(input.error("unable to determine item kind"));
        };

        let Some(item_ident) = item_ident else {
            return Err(input.error("unable to determine item ident"));
        };

        if kind == ItemKind::Fn || kind == ItemKind::Static {
            let span = item_ident.span();
            let mut export_name = proc2::Literal::string(&format!("rust_{item_ident}"));
            export_name.set_span(span);

            attrs.extend([
                TokenTree::Punct(proc2::Punct::new('#', Spacing::Alone)),
                proc_macro2::TokenTree::Group(new_cfg_attr(
                    span,
                    [
                        TokenTree::Ident(Ident::new("feature", span)),
                        TokenTree::Punct(proc2::Punct::new('=', Spacing::Alone)),
                        TokenTree::Literal(proc2::Literal::string("sys")),
                    ],
                    [
                        TokenTree::Ident(Ident::new("unsafe", span)),
                        TokenTree::Group(new_group([
                            TokenTree::Ident(Ident::new("export_name", span)),
                            TokenTree::Punct(proc2::Punct::new('=', Spacing::Alone)),
                            TokenTree::Literal(export_name),
                        ])),
                    ],
                )),
            ]);
        }

        Ok(FfiItem { attrs, code })
    }
}

fn new_cfg_attr<I, J>(span: proc2::Span, condition: I, attr: J) -> Group
where
    I: IntoIterator<Item = TokenTree>,
    J: IntoIterator<Item = TokenTree>,
{
    Group::new(
        Delimiter::Bracket,
        [
            TokenTree::Ident(Ident::new("cfg_attr", span)),
            TokenTree::Group(proc2::Group::new(
                Delimiter::Parenthesis,
                condition
                    .into_iter()
                    .chain([TokenTree::Punct(proc2::Punct::new(',', Spacing::Alone))])
                    .chain(attr)
                    .collect(),
            )),
        ]
        .into_iter()
        .collect(),
    )
}

fn new_group<I>(stream: I) -> Group
where
    I: IntoIterator<Item = TokenTree>,
{
    Group::new(Delimiter::Parenthesis, stream.into_iter().collect())
}
