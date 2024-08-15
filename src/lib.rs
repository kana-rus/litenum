#![doc(html_root_url = "https://docs.rs/litenum/1.0.0/litenum")]
#![doc = include_str!("../README.md")]

mod internal;
mod components;


/// Generate method `fn to_lit(&self) -> &'static str` that converts
/// enum variants to literals by names.
/// 
/// Available for only simple enums with **no** generics or variant fields.
/// 
/// <br/>
/// 
/// ```rust
/// #[litenum::to]
/// enum AnkerTarget {
///     _blank,
///     _self,
///     _top,
///     _parent,
/// }
/// 
/// fn main() {
///     assert_eq!(
///         AnkerTarget::_blank.lit(),
///         "_blank",
///     )
/// }
/// ```
#[proc_macro_attribute]
pub fn to(_: proc_macro::TokenStream, enum_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::to(enum_tokens.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Generate method `fn from_lit(lit: &str) -> Option<Self>` that converts
/// `&str` variables to enum variants by names.
/// 
/// Available for only simple enums with **no** generics or variant fields.
/// 
/// <br/>
/// 
/// ```rust
/// #[litenum::from]
/// #[derive(Debug, PartialEq)]
/// enum AnkerTarget {
///     _blank,
///     _self,
///     _top,
///     _parent,
/// }
/// 
/// fn main() {
///     assert_eq!(
///         AnkerTarget::from_lit("_blank"),
///         Some(AnkerTarget::_blank),
///     )
/// }
/// ```
#[proc_macro_attribute]
pub fn from(_: proc_macro::TokenStream, enum_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::from(enum_tokens.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Utility to impl `lit` and `from_lit` at once
/// 
/// <br/>
/// 
/// ```rust
/// #[litenum::ium] // equals to
///                 // `#[litenum::to] #[litenum::from]`
/// #[derive(Debug, PartialEq)]
/// enum AnkerTarget {
///     _blank,
///     _self,
///     _top,
///     _parent,
/// }
/// 
/// fn main() {
///     assert_eq!(
///         AnkerTarget::_blank.lit(),
///         "_blank",
///     );
/// 
///     assert_eq!(
///         AnkerTarget::from_lit("_blank"),
///         Some(AnkerTarget::_blank),
///     );
/// }
/// ```
#[proc_macro_attribute]
pub fn ium(_: proc_macro::TokenStream, enum_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::ium(enum_tokens.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
