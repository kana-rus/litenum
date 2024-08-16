use syn::{parse::Parse, punctuated::Punctuated, token, Result};
use proc_macro2::{Ident, TokenStream};
use quote::quote;


pub(crate) struct SimpleEnum {
    pub(crate) ident:    Ident,
    pub(crate) variants: Punctuated<Ident, token::Comma>,
}

impl Parse for SimpleEnum {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        while input.peek(token::Pound) {
            input.parse::<token::Pound>()?;
            let attr; syn::bracketed!(attr in input);
            attr.parse::<TokenStream>()?;
        }
        if input.peek(token::Pub) {
            input.parse::<token::Pub>()?;
            if input.peek(token::Paren) {
                let scope; syn::parenthesized!(scope in input);
                scope.parse::<TokenStream>()?;
            }
        }
        input.parse::<token::Enum>()?;
        let ident = input.parse::<Ident>()?;
        let variants_buf; syn::braced!(variants_buf in input);
        let variants = variants_buf.parse_terminated::<_, token::Comma>(Ident::parse)?;

        Ok(Self { ident, variants })
    }
}

impl SimpleEnum {
    pub(crate) fn to_lit_impl(&self) -> TokenStream {
        let Self { ident, variants } = self;

        let proc = {
            let arms = variants.iter().map(|ident| {
                quote! { Self::#ident => stringify!(#ident), }
            });

            quote! {
                match *self {
                    #( #arms )*
                }
            }
        };

        let doc = format!("*implemented by litenum*");

        quote! {
            impl #ident {
                #[doc = #doc]
                #[inline]
                pub(crate) const fn lit(&self) -> &'static str {
                    #proc
                }
            }
        }
    }

    pub(crate) fn from_lit_impl(&self) -> TokenStream {
        let Self { ident, variants } = self;

        let proc = {
            let arms = variants.iter().map(|ident| {
                quote! { stringify!(#ident) => Some(Self::#ident), }
            });

            quote! {
                match lit {
                    #( #arms )*
                    _ => None
                }
            }
        };

        let doc = format!("*implemented by litenum*");

        quote! {
            impl #ident {
                #[doc = #doc]
                #[inline]
                pub(crate) fn from_lit(lit: &str) -> Option<Self> {
                    #proc
                }
            }
        }
    }
}
