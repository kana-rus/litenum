use crate::components::SimpleEnum;
use proc_macro2::TokenStream;
use syn::{Result, parse2};
use quote::quote;


pub(super) fn to(enum_tokens: TokenStream) -> Result<TokenStream> {
    let simple_enum = parse2::<SimpleEnum>(enum_tokens.clone())?;
    let to_lit_impl = simple_enum.to_lit_impl();

    Ok(quote!{
        #[allow(non_camel_case_types)]
        #enum_tokens
        #to_lit_impl
    })
}

pub(super) fn from(enum_tokens: TokenStream) -> Result<TokenStream> {
    let simple_enum = parse2::<SimpleEnum>(enum_tokens.clone())?;
    let from_lit_impl = simple_enum.from_lit_impl();

    Ok(quote!{
        #[allow(non_camel_case_types)]
        #enum_tokens
        #from_lit_impl
    })
}

pub(super) fn ium(enum_tokens: TokenStream) -> Result<TokenStream> {
    let simple_enum = parse2::<SimpleEnum>(enum_tokens.clone())?;
    let to_lit_impl   = simple_enum.to_lit_impl();
    let from_lit_impl = simple_enum.from_lit_impl();

    Ok(quote!{
        #[allow(non_camel_case_types)]
        #enum_tokens
        #to_lit_impl
        #from_lit_impl
    })
}
