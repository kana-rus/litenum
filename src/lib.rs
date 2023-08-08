mod internal;

use proc_macro::TokenStream;


#[proc_macro_attribute]
pub fn to(_: TokenStream, enum_tokens: TokenStream) -> TokenStream {
    internal::to(enum_tokens.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn from(_: TokenStream, enum_tokens: TokenStream) -> TokenStream {
    internal::from(enum_tokens.into())
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
