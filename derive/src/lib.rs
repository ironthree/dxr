//! # dxr_derive
//!
//! This crate is an implementation detail of the `dxr` crate, which provides the derive macros.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Type};

/// procedural macro for deriving the `FromDXR` trait for structs
#[proc_macro_derive(FromDXR)]
pub fn from_dxr(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    for param in &mut input.generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(dxr::FromDXR));
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
                        Type::Path(v) => v,
                        _ => unimplemented!("Deriving FromDXR not possible for field: {}", ident),
                    };
                    let ident_str = ident.to_string();
                    field_impls.push(quote! {
                        #ident: <#stype as FromDXR>::from_dxr(map.get(#ident_str)
                            .ok_or_else(|| ValueError::missing_field(#ident_str))?)?,
                    });
                }
            },
            Fields::Unnamed(_) => unimplemented!("Cannot derive FromDXR for tuple structs."),
            Fields::Unit => unimplemented!("Cannot derive FromDXR for unit structs."),
        },
        _ => unimplemented!("FromDXR can not be derived for enums and unions."),
    }

    let mut fields = proc_macro2::TokenStream::new();
    fields.extend(field_impls.into_iter());

    let impl_block = quote! {
        impl #impl_generics ::dxr_shared::FromDXR for #name #ty_generics #where_clause {
            fn from_dxr(value: &::dxr_shared::types::Value) -> Result<#name, ::dxr_shared::ValueError> {
                use ::std::collections::HashMap;
                use ::std::string::String;
                use ::dxr_shared::types::Value;
                use ::dxr_shared::ValueError;

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

    for param in &mut input.generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(dxr::ToDXR));
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
                        Type::Path(v) => v,
                        _ => unimplemented!("Deriving ToDXR not possible for field: {}", ident),
                    };
                    let ident_str = ident.to_string();
                    field_impls.push(quote! {
                        map.insert(String::from(#ident_str), <#stype as ToDXR>::to_dxr(&self.#ident)?);
                    });
                }
            },
            Fields::Unnamed(_) => unimplemented!("Cannot derive ToDXR for tuple structs."),
            Fields::Unit => unimplemented!("Cannot derive ToDXR for unit structs."),
        },
        _ => unimplemented!("ToDXR can not be derived for enums and unions."),
    }

    let mut fields = proc_macro2::TokenStream::new();
    fields.extend(field_impls.into_iter());

    let impl_block = quote! {
        impl #impl_generics ::dxr_shared::ToDXR for #name #ty_generics #where_clause {
            fn to_dxr(&self) -> Result<Value, ::dxr_shared::ValueError> {
                use ::std::collections::HashMap;
                use ::std::string::String;
                use ::dxr_shared::types::Value;

                let mut map: HashMap<String, Value> = HashMap::new();

                #fields

                HashMap::to_dxr(&map)
            }
        }
    };

    proc_macro::TokenStream::from(impl_block)
}
