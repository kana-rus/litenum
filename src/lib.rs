#![doc(html_root_url = "https://docs.rs/litenum")]

#![no_std]
extern crate alloc;

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
///         AnkerTarget::_blank.to_lit(),
///         "_blank",
///     )
/// }
/// ```
#[proc_macro_attribute]
pub fn to(_: proc_macro::TokenStream, enum_tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    internal::to(enum_tokens.into())
        .unwrap_or_else(|e| e.into_compile_error())
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
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}

/// Utility to generate `to_lit` and `from_lit` at once
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
///         AnkerTarget::_blank.to_lit(),
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
        .unwrap_or_else(|e| e.into_compile_error())
        .into()
}
