//#![warn(missing_docs)]
//#![warn(missing_debug_implementations)]

use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Type};

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
                    let ident = field.ident.as_ref().unwrap();
                    let stype = match &field.ty {
                        Type::Path(v) => v,
                        _ => unimplemented!("Deriving FromValue not possible for field: {}", ident),
                    };
                    field_impls.push(
                        // FIXME 1: replace unwrap with a nice error message about a missing field
                        // FIXME 2: replace ? with nice error message about wrong type
                        // FIXME 3: this does not work for types like Option<T> and HashMap<K, V>
                        //          where only Option::from_value and HashMap::from_value should be
                        //          used, but I can't find a way to strip generics from the path
                        quote! { #ident: #stype::from_value(map.get("#ident").unwrap())?,
                        },
                    );
                }
            },
            Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
        },
        _ => unimplemented!(),
    }

    let mut fields = proc_macro2::TokenStream::new();
    fields.extend(field_impls.into_iter());

    let impl_block = quote! {
        impl #impl_generics dxr::FromValue<#name> for #name #ty_generics #where_clause {
            fn from_value(value: &::dxr_shared::Value) -> Result<#name, ()> {
                use ::std::collections::HashMap;
                use ::std::string::String;
                use ::dxr_shared::Value;

                let map: HashMap<String, Value> = HashMap::from_value(value)?;

                Ok(#name {
                    #fields
                })
            }
        }
    };

    proc_macro::TokenStream::from(impl_block)
}

/*
REFERENCE:

The derive macro should produce an impl block like the one below for this struct
(copied from dxr/examples/koji.rs).

#[derive(Debug, FromValue)]
pub struct Build {
    pub build_id: i32,
    //cg_id: Option<?>,
    pub completion_time: String,
    pub completion_ts: f64,
    pub creation_event_id: i32,
    pub creation_time: String,
    pub creation_ts: f64,
    pub epoch: Option<i32>,
    //extra: HashMap<String, Value>,
    pub id: i32,
    pub name: String,
    pub nvr: String,
    pub owner_id: i32,
    pub owner_name: String,
    pub package_id: i32,
    pub package_name: String,
    pub release: String,
    pub source: String,
    pub start_time: String,
    pub start_ts: f64,
    pub state: i32,
    pub task_id: i32,
    pub version: String,
    pub volume_id: i32,
    pub volume_name: String,
    //cg_name: Option<?>,
}

impl FromValue<Build> for Build {
    fn from_value(value: &Value) -> Result<Build, ()> {
        let map: HashMap<String, Value> = HashMap::from_value(value)?;

        Ok(Build {
            build_id: i32::from_value(map.get("build_id").unwrap())?,
            completion_time: String::from_value(map.get("completion_time").unwrap())?,
            completion_ts: f64::from_value(map.get("completion_ts").unwrap())?,
            creation_event_id: i32::from_value(map.get("creation_event_id").unwrap())?,
            creation_time: String::from_value(map.get("creation_time").unwrap())?,
            creation_ts: f64::from_value(map.get("creation_ts").unwrap())?,
            epoch: Option::from_value(map.get("epoch").unwrap())?,
            //extra: HashMap::from_value(map.get("extra").unwrap())?,
            id: i32::from_value(map.get("id").unwrap())?,
            name: String::from_value(map.get("name").unwrap())?,
            nvr: String::from_value(map.get("nvr").unwrap())?,
            owner_id: i32::from_value(map.get("owner_id").unwrap())?,
            owner_name: String::from_value(map.get("owner_name").unwrap())?,
            package_id: i32::from_value(map.get("package_id").unwrap())?,
            package_name: String::from_value(map.get("package_name").unwrap())?,
            release: String::from_value(map.get("release").unwrap())?,
            source: String::from_value(map.get("source").unwrap())?,
            start_time: String::from_value(map.get("start_time").unwrap())?,
            start_ts: f64::from_value(map.get("start_ts").unwrap())?,
            state: i32::from_value(map.get("state").unwrap())?,
            task_id: i32::from_value(map.get("task_id").unwrap())?,
            version: String::from_value(map.get("version").unwrap())?,
            volume_id: i32::from_value(map.get("volume_id").unwrap())?,
            volume_name: String::from_value(map.get("volume_name").unwrap())?,
        })
    }
}
*/
