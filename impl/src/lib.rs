//! This crate implements the macro for `nova` and should not be used directly.

use std::ops::Deref;
use std::{cmp::Ordering, collections::HashSet, iter::FromIterator};

use darling::util::PathList;
use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    AttributeArgs, GenericArgument, Token, TypePath,
};

#[doc(hidden)]
#[derive(Debug, Default, FromMeta)]
pub struct Attrs {
    #[darling(default)]
    new: bool,
    #[darling(default)]
    copy: bool,
    #[darling(default)]
    opaque: bool,
    #[darling(default)]
    serde: bool,
    #[darling(default)]
    sqlx: bool,
    #[darling(default)]
    async_graphql: bool,
    #[darling(default)]
    borrow: Option<syn::Path>,
    #[darling(default)]
    try_from: Option<syn::LitStr>,
    #[darling(default)]
    display: bool,

    #[darling(default)]
    derive: Option<PathList>,
}

fn pointy_bits(ty: &syn::Type) -> Punctuated<GenericArgument, Token![,]> {
    let set = match ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .iter()
            .map(|x| match &x.arguments {
                syn::PathArguments::AngleBracketed(a) => {
                    a.args.iter().map(|x| x).cloned().collect()
                }
                syn::PathArguments::Parenthesized(_) => vec![],
                syn::PathArguments::None => vec![],
            })
            .flatten()
            .collect::<HashSet<_>>(),
        _ => Default::default(),
    };

    let mut vec = set.into_iter().collect::<Vec<_>>();
    vec.sort_by(|a, b| {
        if a == b {
            return Ordering::Equal;
        }

        match (a, b) {
            (GenericArgument::Lifetime(_), _) => Ordering::Greater,
            (GenericArgument::Type(_), GenericArgument::Lifetime(_)) => Ordering::Less,
            (GenericArgument::Type(_), GenericArgument::Const(_)) => Ordering::Greater,
            (GenericArgument::Const(_), _) => Ordering::Less,
            _ => Ordering::Less,
        }
    });

    Punctuated::from_iter(vec.into_iter())
}

#[doc(hidden)]
#[derive(Debug, Default, FromMeta)]
pub struct SerdeAttrs {
    #[allow(dead_code)]
    #[darling(default, rename = "crate")]
    crate_: Option<syn::Path>,
}

fn do_newtype(mut attrs: Attrs, item: Item) -> Result<TokenStream, syn::Error> {
    let Item {
        visibility,
        new_ty,
        wrapped_ty,
    } = item;

    let borrow_ty = attrs
        .borrow
        .take()
        .map(|path| syn::Type::Path(TypePath { qself: None, path }))
        .unwrap_or_else(|| wrapped_ty.clone());

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
        let segments = match &wrapped_ty {
            syn::Type::Path(p) => &p.path.segments,
            _ => panic!("Ahhhh"),
        };

        let sql_type_literal = match &*segments.last().unwrap().ident.to_string() {
            "u128" | "i128" | "Uuid" => "UUID",
            "u64" | "i64" => "INT8",
            "u32" | "i32" => "INT4",
            "u16" | "i16" | "u8" | "i8" => "INT2",
            "bool" => "BOOL",
            _ => "",
        };

        let sql_type_literal = if sql_type_literal != "" {
            quote! { #[sqlx(transparent, type_name = #sql_type_literal)] }
        } else {
            quote! { #[sqlx(transparent)] }
        };

        quote! {
            #[derive(sqlx::Type)]
            #sql_type_literal
        }
    } else {
        // sqlx's derive interferes with a repr declaration, so we do it here.
        quote! {
            #[repr(transparent)]
        }
    };

    let async_graphql = if attrs.async_graphql {
        Some(quote! {
            async_graphql::scalar!(#new_ty);
        })
    } else {
        None
    };

    let pointy_bits = pointy_bits(&new_ty);
    let pointy = quote!( < #pointy_bits > );

    let deref = if attrs.opaque {
        None
    } else {
        Some(quote! {
            impl #pointy core::ops::Deref for #new_ty {
                type Target = #borrow_ty;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl #pointy #new_ty {
                #[allow(dead_code)]
                pub fn into_inner(self) -> #wrapped_ty {
                    self.0
                }
            }
        })
    };

    let new = if attrs.new {
        let consty = if attrs.copy {
            Some(quote! { const })
        } else {
            None
        };
        Some(quote! {
            impl #pointy #new_ty {
                pub #consty fn new(input: #wrapped_ty) -> Self {
                    Self(input)
                }
            }

            impl #pointy From<#wrapped_ty> for #new_ty {
                fn from(x: #wrapped_ty) -> Self {
                    Self(x)
                }
            }
        })
    } else {
        None
    };

    let trait_impl = quote! {
        impl #pointy ::nova::NewType for #new_ty {
            type Inner = #wrapped_ty;
        }
    };

    let display = if attrs.display {
        Some(quote! {
            impl #pointy core::fmt::Display for #new_ty {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    core::fmt::Display::fmt(&self.0, f)
                }
            }
        })
    } else {
        None
    };

    let derives = if let Some(custom_derives) = attrs.derive {
        let paths = custom_derives.deref().clone();
        quote! { #[derive( #(#paths),*)]}
    } else {
        quote! { #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, core::hash::Hash)]}
    };
    let out = quote! {
        #derives
        #copy
        #serde
        #sqlx
        #visibility struct #new_ty(#wrapped_ty);
        #async_graphql
        #deref
        #new
        #trait_impl
        #display
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
    new_ty: syn::Type,
    wrapped_ty: syn::Type,
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

        let new_ty: syn::Type = input.parse()?;
        let _: Token![=] = input.parse()?;
        let wrapped_ty: syn::Type = input.parse()?;
        let _: Token![;] = input.parse()?;

        // println!("{:?}", input.cursor().token_stream().to_string());

        Ok(Item {
            visibility,
            new_ty,
            wrapped_ty,
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
                quote! { pub(crate) type Hello = u8; },
            )
            .unwrap()
        );

        println!(
            "{:?}",
            newtype(
                vec![syn::parse_quote!(copy)],
                quote! { pub(in super) type SpecialUuid = uuid::Uuid; },
            )
            .unwrap()
        );

        println!(
            "{:?}",
            newtype(
                vec![syn::parse_quote!(new), syn::parse_quote!(borrow = "str")],
                quote! { pub(in super) type S<'a> = std::borrow::Cow<'a, str>; },
            )
            .unwrap()
        );
    }
}
