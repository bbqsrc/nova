//! This crate implements the macro for `nova` and should not be used directly.

extern crate proc_macro;

use darling::{ast::NestedMeta, Error};
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_attribute]
/// Create a newtype from a `type` declaration.
pub fn newtype(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let item = parse_macro_input!(item as proc_macro2::TokenStream);

    match nova_impl::newtype(&attrs, item) {
        Ok(tokens) => tokens.into(),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
