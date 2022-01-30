//#![warn(missing_docs)]
//#![warn(missing_debug_implementations)]

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam};

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

    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                for field in &fields.named {
                    let _name = field.ident.as_ref().unwrap();
                    todo!();
                }
            },
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },
        _ => unimplemented!(),
    }

    let expanded = quote! {
        impl #impl_generics dxr::FromValue for #name #ty_generics #where_clause {
            fn from_value(dxr_shared::Value) -> #name {
                todo!()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
