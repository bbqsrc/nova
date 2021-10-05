//! This crate implements the macro for `nova` and should not be used directly.

use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    AttributeArgs, Token, TypePath,
};

#[doc(hidden)]
#[derive(Debug, Default, FromMeta)]
pub struct Attrs {
    #[darling(default)]
    copy: bool,
    #[darling(default)]
    opaque: bool,
    #[darling(default)]
    serde: bool,
    #[darling(default)]
    sqlx: bool,
    #[darling(default)]
    borrow: Option<syn::Path>,
    #[darling(default)]
    try_from: Option<syn::LitStr>,
}

#[doc(hidden)]
#[derive(Debug, Default, FromMeta)]
pub struct SerdeAttrs {
    #[darling(default, rename = "crate")]
    crate_: Option<syn::Path>,
}

fn do_newtype(mut attrs: Attrs, item: Item) -> Result<TokenStream, syn::Error> {
    let Item {
        visibility,
        ident,
        ty,
    } = item;

    let borrow_ty = attrs
        .borrow
        .take()
        .map(|path| syn::Type::Path(TypePath { qself: None, path }))
        .unwrap_or_else(|| ty.clone());

    let copy = if attrs.copy {
        Some(quote! {
            #[derive(Copy)]
        })
    } else {
        None
    };

    let serde = if attrs.serde {
        let serde_path: syn::Path = syn::parse_quote! { serde };
        Some(match attrs.try_from.as_ref() {
            Some(path) => {
                quote! {
                    #[derive(#serde_path::Deserialize, #serde_path::Serialize)]
                    #[serde(try_from = #path)]
                }
            }
            None => quote! {
                #[derive(#serde_path::Deserialize, #serde_path::Serialize)]
                #[serde(transparent)]
            },
        })
    } else {
        None
    };

    let sqlx = if attrs.sqlx {
        quote! {
            #[derive(sqlx::Type)]
            #[sqlx(transparent)]
        }
    } else {
        // sqlx's derive interferes with a repr declaration, so we do it here.
        quote! {
            #[repr(transparent)]
        }
    };

    let deref = if attrs.opaque {
        None
    } else {
        Some(quote! {
            impl core::ops::Deref for #ident {
                type Target = #borrow_ty;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl #ident {
                #[allow(dead_code)]
                pub fn into_inner(self) -> #ty {
                    self.0
                }
            }
        })
    };

    let out = quote! {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, core::hash::Hash)]
        #copy
        #serde
        #sqlx
        #visibility struct #ident(#ty);
        #deref
    };

    Ok(out)
}

#[doc(hidden)]
pub fn newtype(attrs: AttributeArgs, item: TokenStream) -> Result<TokenStream, syn::Error> {
    let attrs = match Attrs::from_list(&attrs) {
        Ok(v) => v,
        Err(e) => {
            return Ok(TokenStream::from(e.write_errors()));
        }
    };

    let item: Item = syn::parse2(item.clone())?;

    do_newtype(attrs, item)
}

#[derive(Debug)]
struct Item {
    visibility: syn::Visibility,
    ident: syn::Ident,
    ty: syn::Type,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();

        let visibility = if lookahead.peek(Token![pub]) {
            let visibility: syn::Visibility = input.call(syn::Visibility::parse)?;
            visibility
        } else {
            syn::Visibility::Inherited
        };

        let _: Token![type] = input.parse()?;

        let ident: Ident = input.parse()?;
        let _: Token![=] = input.parse()?;
        let ty: syn::Type = input.parse()?;
        let _: Token![;] = input.parse()?;

        // println!("{:?}", input.cursor().token_stream().to_string());

        Ok(Item {
            visibility,
            ty,
            ident,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        println!(
            "{:?}",
            newtype(
                vec![syn::parse_quote!(copy)],
                quote! { pub(crate) type Hello = u8; }
            )
        );

        println!(
            "{:?}",
            newtype(
                vec![syn::parse_quote!(copy)],
                quote! { pub(in super) type SpecialUuid = uuid::Uuid; }
            )
        );
    }
}
