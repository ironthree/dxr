#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(unsafe_code)]
#![warn(explicit_outlives_requirements)]
#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unreachable_pub)]
#![warn(clippy::unwrap_used)]

//! # dxr_derive
//!
//! This crate is an implementation detail of the `dxr` crate, which provides the derive macros.

use proc_macro::TokenStream;

use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Ident, Type};

fn use_dxr() -> TokenStream2 {
    let krate = crate_name("dxr").ok().unwrap_or(FoundCrate::Itself);

    match krate {
        FoundCrate::Itself => quote! { crate },
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { #ident }
        },
    }
}

/// procedural macro for deriving the `TryFromValue` trait for structs
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
#[proc_macro_derive(TryFromValue)]
pub fn try_from_value(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let name_str = name.to_string();
    let dxr = use_dxr();

    for param in &mut input.generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(#dxr::TryFromValue));
        }
    }

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut field_impls = Vec::new();

    match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    for field in &fields.named {
                        let ident = field.ident.as_ref().expect("Failed to get struct field identifier.");
                        let stype = match &field.ty {
                        Type::Path(t) => t.to_token_stream(),
                        Type::Tuple(t) => t.to_token_stream(),
                        // syn::Type::Array: fixed-size array
                        Type::Array(t) => t.to_token_stream(),
                        // syn::Type::Slice: dynamically-sized array
                        Type::Slice(_) => return quote_spanned! {
                            field.ty.span() => compile_error!(
                                "Deriving TryFromValue is not possible for structs that contain dynamically sized arrays, \
                                 as they don't implement Sized. Try using a Vec here."
                            );
                        }.into(),
                        Type::Reference(_) => return quote_spanned! {
                            field.ty.span() => compile_error!(
                                "Deriving TryFromValue is not possible for structs that contain reference types. \
                                 Try using a std::borrow::Cow here."
                            );
                        }.into(),
                        _ => return quote_spanned! {
                            field.ty.span() => compile_error!(
                                "Deriving TryFromValue is not possible due to an unrecognised struct field type."
                            );
                        }.into(),
                    };
                        let ident_str = ident.to_string();
                        field_impls.push(quote! {
                            #ident: <#stype as TryFromValue>::try_from_value(map.get(#ident_str)
                                .ok_or_else(|| #dxr::DxrError::missing_field(#name_str, #ident_str))?)?,
                        });
                    }
                },
                Fields::Unnamed(_) => {
                    return quote_spanned! {
                        name.span() => compile_error!(
                            "Deriving TryFromValue for tuple structs is not supported."
                        );
                    }
                    .into()
                },
                Fields::Unit => {
                    return quote_spanned! {
                        name.span() => compile_error!(
                            "Deriving TryFromValue for unit structs is not supported."
                        );
                    }
                    .into()
                },
            }
        },
        Data::Enum(_) | Data::Union(_) => {
            return quote_spanned! {
                name.span() => compile_error!(
                    "Deriving TryFromValue for enums and unions is not supported."
                );
            }
            .into()
        },
    };

    let mut fields = TokenStream2::new();
    fields.extend(field_impls.into_iter());

    let impl_block = quote! {
        impl #impl_generics #dxr::TryFromValue for #name #ty_generics #where_clause {
            fn try_from_value(value: &#dxr::Value) -> Result<#name #ty_generics, #dxr::DxrError> {
                use ::std::collections::HashMap;
                use ::std::string::String;
                use #dxr::{Value, DxrError};

                let map: HashMap<String, Value> = HashMap::try_from_value(value)?;

                Ok(#name {
                    #fields
                })
            }
        }
    };

    proc_macro::TokenStream::from(impl_block)
}

/// procedural macro for deriving the `TryToValue` trait for structs
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
#[proc_macro_derive(TryToValue)]
pub fn try_to_value(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let dxr = use_dxr();

    for param in &mut input.generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(#dxr::TryToValue));
        }
    }

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut field_impls = Vec::new();

    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    let ident = field.ident.as_ref().expect("Failed to get struct field identifier.");
                    let stype =
                        match &field.ty {
                            Type::Path(t) => t.to_token_stream(),
                            Type::Tuple(t) => t.to_token_stream(),
                            Type::Reference(t) => t.to_token_stream(),
                            // syn::Type::Array: fixed-size array
                            Type::Array(t) => t.to_token_stream(),
                            // syn::Type::Slice: dynamically-sized array
                            Type::Slice(_) => return quote_spanned! {
                                field.ty.span() => compile_error!(
                                    "Deriving TryToValue is not possible for structs that contain dynamically sized arrays, \
                                     as they don't implement Sized. Try using a Vec or slice reference here."
                                );
                            }
                            .into(),
                            _ => {
                                return quote_spanned! {
                                    field.ty.span() => compile_error!(
                                        "Deriving TryToValue is not possible due to an unrecognised struct field type."
                                    );
                                }
                                .into()
                            },
                        };
                    let ident_str = ident.to_string();
                    field_impls.push(quote! {
                        map.insert(String::from(#ident_str), <#stype as TryToValue>::try_to_value(&self.#ident)?);
                    });
                }
            },
            Fields::Unnamed(_) => {
                return quote_spanned! {
                    name.span() => compile_error!(
                        "Deriving TryToValue for tuple structs is not supported."
                    );
                }
                .into()
            },
            Fields::Unit => {
                return quote_spanned! {
                    name.span() => compile_error!(
                        "Deriving TryToValue for unit structs is not supported."
                    );
                }
                .into()
            },
        },
        Data::Enum(_) | Data::Union(_) => {
            return quote_spanned! {
                name.span() => compile_error!(
                    "Deriving TryToValue for enums and unions is not supported."
                );
            }
            .into()
        },
    }

    let mut fields = TokenStream2::new();
    fields.extend(field_impls.into_iter());

    let impl_block = quote! {
        impl #impl_generics #dxr::TryToValue for #name #ty_generics #where_clause {
            fn try_to_value(&self) -> Result<#dxr::Value, #dxr::DxrError> {
                use ::std::collections::HashMap;
                use ::std::string::String;
                use #dxr::Value;

                let mut map: HashMap<String, Value> = HashMap::new();

                #fields

                HashMap::try_to_value(&map)
            }
        }
    };

    proc_macro::TokenStream::from(impl_block)
}
