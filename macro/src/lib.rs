//! This crate implements the macro for `nova` and should not be used directly.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};

#[proc_macro_attribute]
/// Create a newtype from a `type` declaration.
pub fn newtype(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as proc_macro2::TokenStream);

    match nova_impl::newtype(attrs, item) {
        Ok(tokens) => tokens.into(),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
