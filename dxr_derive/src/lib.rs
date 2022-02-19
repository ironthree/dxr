#![deny(unsafe_code)]
#![warn(explicit_outlives_requirements)]
#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(clippy::unwrap_used)]

//! # dxr_derive
//!
//! This crate is an implementation detail of the `dxr` crate, which provides the derive macros.

use proc_macro::TokenStream;

use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, ToTokens};
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

/// procedural macro for deriving the `FromDXR` trait for structs
#[proc_macro_derive(FromDXR)]
pub fn from_dxr(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let name_str = name.to_string();
    let dxr = use_dxr();

    for param in &mut input.generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(#dxr::FromDXR));
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
                    let stype = match &field.ty {
                        Type::Path(t) => t.to_token_stream(),
                        Type::Tuple(t) => t.to_token_stream(),
                        Type::Array(_) => panic!(
                            "Deriving FromDXR is not possible for structs that contain array types. \
                             Try using a Vec for field \"{}\" instead.",
                            ident
                        ),
                        Type::Reference(_) => panic!(
                            "Deriving FromDXR is not possible for structs that contain reference types. \
                             Try using a std::borrow::Cow for field \"{}\" instead.",
                            ident
                        ),
                        _ => panic!("Deriving FromDXR not possible for field: {}", ident),
                    };
                    let ident_str = ident.to_string();
                    field_impls.push(quote! {
                        #ident: <#stype as FromDXR>::from_dxr(map.get(#ident_str)
                            .ok_or_else(|| DxrError::missing_field(#name_str, #ident_str))?)?,
                    });
                }
            },
            Fields::Unnamed(_) => panic!("Deriving FromDXR for tuple structs is not possible."),
            Fields::Unit => panic!("Deriving FromDXR for unit structs is not possible."),
        },
        _ => panic!("Deriving FromDXR for enums and unions is not supported."),
    }

    let mut fields = TokenStream2::new();
    fields.extend(field_impls.into_iter());

    let impl_block = quote! {
        impl #impl_generics #dxr::FromDXR for #name #ty_generics #where_clause {
            fn from_dxr(value: &#dxr::Value) -> Result<#name #ty_generics, #dxr::DxrError> {
                use ::std::collections::HashMap;
                use ::std::string::String;
                use #dxr::{Value, DxrError};

                let map: HashMap<String, Value> = HashMap::from_dxr(value)?;

                Ok(#name {
                    #fields
                })
            }
        }
    };

    proc_macro::TokenStream::from(impl_block)
}

/// procedural macro for deriving the `ToDXR` trait for structs
#[proc_macro_derive(ToDXR)]
pub fn to_dxr(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let dxr = use_dxr();

    for param in &mut input.generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(#dxr::ToDXR));
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
                    let stype = match &field.ty {
                        Type::Path(t) => t.to_token_stream(),
                        Type::Tuple(t) => t.to_token_stream(),
                        Type::Array(t) => t.to_token_stream(),
                        Type::Reference(t) => t.to_token_stream(),
                        _ => panic!("Deriving ToDXR not possible for field: {}", ident),
                    };
                    let ident_str = ident.to_string();
                    field_impls.push(quote! {
                        map.insert(String::from(#ident_str), <#stype as ToDXR>::to_dxr(&self.#ident)?);
                    });
                }
            },
            Fields::Unnamed(_) => panic!("Deriving ToDXR for tuple structs is not possible."),
            Fields::Unit => panic!("Deriving ToDXR for unit structs is not possible."),
        },
        _ => panic!("Deriving ToDXR for enums and unions is not supported."),
    }

    let mut fields = TokenStream2::new();
    fields.extend(field_impls.into_iter());

    let impl_block = quote! {
        impl #impl_generics #dxr::ToDXR for #name #ty_generics #where_clause {
            fn to_dxr(&self) -> Result<#dxr::Value, #dxr::DxrError> {
                use ::std::collections::HashMap;
                use ::std::string::String;
                use #dxr::Value;

                let mut map: HashMap<String, Value> = HashMap::new();

                #fields

                HashMap::to_dxr(&map)
            }
        }
    };

    proc_macro::TokenStream::from(impl_block)
}
