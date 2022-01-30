//! # dxr_derive
//!
//! This crate is an implementation detail of the `dxr` crate, which provides the derive macros.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Type};

/// procedural macro for deriving the `FromValue` trait for structs
#[proc_macro_derive(FromValue)]
pub fn from_value(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    for param in &mut input.generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(dxr::FromValue));
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
                        _ => unimplemented!("Deriving FromValue not possible for field: {}", ident),
                    };
                    let ident_str = ident.to_string();
                    field_impls.push(quote! {
                        #ident: <#stype as FromValue<#stype>>::from_value(map.get(#ident_str)
                            .ok_or_else(|| ValueError::missing_field(#ident_str))?)?,
                    });
                }
            },
            Fields::Unnamed(_) => unimplemented!("Cannot derive FromValue for tuple structs."),
            Fields::Unit => unimplemented!("Cannot derive FromValue for unit structs."),
        },
        _ => unimplemented!("FromValue can not be derived for enums and unions."),
    }

    let mut fields = proc_macro2::TokenStream::new();
    fields.extend(field_impls.into_iter());

    let impl_block = quote! {
        impl #impl_generics ::dxr_shared::FromValue<#name> for #name #ty_generics #where_clause {
            fn from_value(value: &::dxr_shared::Value) -> Result<#name, ::dxr_shared::ValueError> {
                use ::std::collections::HashMap;
                use ::std::string::String;
                use ::dxr_shared::Value;
                use ::dxr_shared::ValueError;

                let map: HashMap<String, Value> = HashMap::from_value(value)?;

                Ok(#name {
                    #fields
                })
            }
        }
    };

    proc_macro::TokenStream::from(impl_block)
}

/// procedural macro for deriving the `ToValue` trait for structs
#[proc_macro_derive(ToValue)]
pub fn to_value(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    for param in &mut input.generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(dxr::FromValue));
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
                        _ => unimplemented!("Deriving FromValue not possible for field: {}", ident),
                    };
                    let ident_str = ident.to_string();
                    field_impls.push(quote! {
                        map.insert(String::from(#ident_str), <#stype as ToValue<#stype>>::to_value(&value.#ident));
                    });
                }
            },
            Fields::Unnamed(_) => unimplemented!("Cannot derive FromValue for tuple structs."),
            Fields::Unit => unimplemented!("Cannot derive FromValue for unit structs."),
        },
        _ => unimplemented!("FromValue can not be derived for enums and unions."),
    }

    let mut fields = proc_macro2::TokenStream::new();
    fields.extend(field_impls.into_iter());

    let impl_block = quote! {
        impl #impl_generics ::dxr_shared::ToValue<#name> for #name #ty_generics #where_clause {
            fn to_value(value: &#name) -> Value {
                use ::std::collections::HashMap;
                use ::std::string::String;
                use ::dxr_shared::Value;

                let mut map: HashMap<String, Value> = HashMap::new();

                #fields

                HashMap::to_value(&map)
            }
        }
    };

    proc_macro::TokenStream::from(impl_block)
}
