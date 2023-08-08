use proc_macro2::{TokenStream, Span};
use syn::{Result, parse2, ItemEnum, Error};

pub(super) fn to(enum_tokens: TokenStream) -> Result<TokenStream> {
    let ItemEnum {
        attrs,
        vis,
        enum_token,
        ident,
        generics,
        brace_token,
        variants,
    } = parse2(enum_tokens)?;

    if variants.iter().any(|v| !v.fields.is_empty()) {
        return Err(Error::new(Span::call_site(), ""))
    }

    
}

pub(super) fn from(enum_tokens: TokenStream) -> Result<TokenStream> {
    todo!()
}
