use proc_macro2::{Span, Ident, TokenStream};
use quote::quote;
use syn::{ItemEnum, Result, Error, punctuated::Punctuated, Variant, token::Comma, parse::Parse};


pub(crate) struct SimpleEnum {
    pub(crate) ident:    Ident,
    pub(crate) variants: Punctuated<Variant, Comma>,
} impl Parse for SimpleEnum {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let ItemEnum { ident, generics, variants, .. } = input.parse::<ItemEnum>()?;

        if !generics.params.is_empty() {
            return Err(Error::new(Span::call_site(), "Generics isn't supported"))
        }
        if variants.iter().any(|v| !v.fields.is_empty()) {
            return Err(Error::new(Span::call_site(), "Variant with fields isn't supported"))
        }
    
        Ok(Self { ident, variants })
    }
} impl SimpleEnum {
    pub(crate) fn to_lit_impl(&self) -> TokenStream {
        let Self { ident, variants } = self;

        let arms = variants.iter().map(|v| {
            let ident     = &v.ident;
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

        let arms = variants.iter().map(|v| {
            let ident     = &v.ident;
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
