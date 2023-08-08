use alloc::{string::ToString};

use quote::{quote};
use proc_macro2::{Ident, TokenStream};
use syn::{Result, punctuated::Punctuated, token, parse::Parse, braced, bracketed};


pub(crate) struct SimpleEnum {
    pub(crate) ident:    Ident,
    pub(crate) variants: Punctuated<Ident, token::Comma>,
} impl Parse for SimpleEnum {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        while input.peek(token::Pound) {
            input.parse::<token::Pound>()?;
            let attr; bracketed!(attr in input);
            attr.parse::<TokenStream>()?;
        }
        input.parse::<token::Enum>()?;
        let ident = input.parse::<Ident>()?;
        let variants_buf; braced!(variants_buf in input);
        let variants = variants_buf.parse_terminated::<_, token::Comma>(Ident::parse)?;

        Ok(Self { ident, variants })
    }
} impl SimpleEnum {
    pub(crate) fn to_lit_impl(&self) -> TokenStream {
        let Self { ident, variants } = self;

        let arms = variants.iter().map(|ident| {
            let ident_str = ident.to_string();
            quote!{
                Self::#ident => #ident_str,
            }
        });

        quote!{
            impl #ident {
                #[inline] pub(crate) fn to_lit(&self) -> &'static str {
                    match self {
                        #( #arms )*
                    }
                }
            }
        }
    }

    pub(crate) fn from_lit_impl(&self) -> TokenStream {
        let Self { ident, variants } = self;

        let arms = variants.iter().map(|ident| {
            let ident_str = ident.to_string();
            quote!{
                #ident_str => Some(Self::#ident),
            }
        });

        quote!{
            impl #ident {
                #[inline] pub(crate) fn from_lit(lit: &str) -> Option<Self> {
                    match lit {
                        #( #arms )*
                        _ => None
                    }
                }
            }
        }
    }
}
