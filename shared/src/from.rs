use std::collections::HashMap;

use chrono::{DateTime, Utc};
use quick_xml::escape::unescape;

use crate::{types::Type, DxrError, FromDXR, Value};

impl FromDXR for Value {
    fn from_dxr(value: &Value) -> Result<Value, DxrError> {
        Ok(value.clone())
    }
}

impl FromDXR for i32 {
    fn from_dxr(value: &Value) -> Result<i32, DxrError> {
        let err = |t: &'static str| DxrError::wrong_type(t, "i4");

        match value.inner() {
            Type::Integer(int) => Ok(*int),
            #[cfg(feature = "i8")]
            Type::Long(_) => Err(err("i8")),
            Type::Boolean(_) => Err(err("boolean")),
            Type::String(_) => Err(err("string")),
            Type::Double(_) => Err(err("double")),
            Type::DateTime(_) => Err(err("dateTime.iso8861")),
            Type::Base64(_) => Err(err("base64")),
            Type::Struct { .. } => Err(err("struct")),
            Type::Array { .. } => Err(err("array")),
            #[cfg(feature = "nil")]
            Type::Nil => Err(err("nil")),
        }
    }
}

#[cfg(feature = "i8")]
impl FromDXR for i64 {
    fn from_dxr(value: &Value) -> Result<i64, DxrError> {
        let err = |t: &'static str| DxrError::wrong_type(t, "i8");

        match value.inner() {
            Type::Integer(_) => Err(err("i4")),
            Type::Long(long) => Ok(*long),
            Type::Boolean(_) => Err(err("boolean")),
            Type::String(_) => Err(err("string")),
            Type::Double(_) => Err(err("double")),
            Type::DateTime(_) => Err(err("dateTime.iso8861")),
            Type::Base64(_) => Err(err("base64")),
            Type::Struct { .. } => Err(err("struct")),
            Type::Array { .. } => Err(err("array")),
            #[cfg(feature = "nil")]
            Type::Nil => Err(err("nil")),
        }
    }
}

impl FromDXR for bool {
    fn from_dxr(value: &Value) -> Result<bool, DxrError> {
        let err = |t: &'static str| DxrError::wrong_type(t, "boolean");

        match value.inner() {
            Type::Integer(_) => Err(err("i4")),
            #[cfg(feature = "i8")]
            Type::Long(_) => Err(err("i8")),
            Type::Boolean(boo) => Ok(*boo),
            Type::String(_) => Err(err("string")),
            Type::Double(_) => Err(err("double")),
            Type::DateTime(_) => Err(err("dateTime.iso8861")),
            Type::Base64(_) => Err(err("base64")),
            Type::Struct { .. } => Err(err("struct")),
            Type::Array { .. } => Err(err("array")),
            #[cfg(feature = "nil")]
            Type::Nil => Err(err("nil")),
        }
    }
}

impl FromDXR for String {
    fn from_dxr(value: &Value) -> Result<String, DxrError> {
        let err = |t: &'static str| DxrError::wrong_type(t, "string");

        match value.inner() {
            Type::Integer(_) => Err(err("i4")),
            #[cfg(feature = "i8")]
            Type::Long(_) => Err(err("i8")),
            Type::Boolean(_) => Err(err("boolean")),
            Type::String(string) => match unescape(string.as_bytes()) {
                Ok(bytes) => String::from_utf8(bytes.to_vec()).map_err(|error| DxrError::InvalidData {
                    error: error.to_string(),
                }),
                Err(error) => Err(DxrError::InvalidData {
                    error: error.to_string(),
                }),
            },
            Type::Double(_) => Err(err("double")),
            Type::DateTime(_) => Err(err("dateTime.iso8861")),
            Type::Base64(_) => Err(err("base64")),
            Type::Struct { .. } => Err(err("struct")),
            Type::Array { .. } => Err(err("array")),
            #[cfg(feature = "nil")]
            Type::Nil => Err(err("nil")),
        }
    }
}

impl FromDXR for f64 {
    fn from_dxr(value: &Value) -> Result<f64, DxrError> {
        let err = |t: &'static str| DxrError::wrong_type(t, "double");

        match value.inner() {
            Type::Integer(_) => Err(err("i4")),
            #[cfg(feature = "i8")]
            Type::Long(_) => Err(err("i8")),
            Type::Boolean(_) => Err(err("boolean")),
            Type::String(_) => Err(err("string")),
            Type::Double(double) => Ok(*double),
            Type::DateTime(_) => Err(err("dateTime.iso8861")),
            Type::Base64(_) => Err(err("base64")),
            Type::Struct { .. } => Err(err("struct")),
            Type::Array { .. } => Err(err("array")),
            #[cfg(feature = "nil")]
            Type::Nil => Err(err("nil")),
        }
    }
}

impl FromDXR for DateTime<Utc> {
    fn from_dxr(value: &Value) -> Result<DateTime<Utc>, DxrError> {
        let err = |t: &'static str| DxrError::wrong_type(t, "dateTime.iso8861");

        match value.inner() {
            Type::Integer(_) => Err(err("i4")),
            #[cfg(feature = "i8")]
            Type::Long(_) => Err(err("i8")),
            Type::Boolean(_) => Err(err("boolean")),
            Type::String(_) => Err(err("string")),
            Type::Double(_) => Err(err("double")),
            Type::DateTime(date) => Ok(*date),
            Type::Base64(_) => Err(err("base64")),
            Type::Struct { .. } => Err(err("struct")),
            Type::Array { .. } => Err(err("array")),
            #[cfg(feature = "nil")]
            Type::Nil => Err(err("nil")),
        }
    }
}

impl FromDXR for Vec<u8> {
    fn from_dxr(value: &Value) -> Result<Vec<u8>, DxrError> {
        let err = |t: &'static str| DxrError::wrong_type(t, "base64");

        match value.inner() {
            Type::Integer(_) => Err(err("i4")),
            #[cfg(feature = "i8")]
            Type::Long(_) => Err(err("i8")),
            Type::Boolean(_) => Err(err("boolean")),
            Type::String(_) => Err(err("string")),
            Type::Double(_) => Err(err("double")),
            Type::DateTime(_) => Err(err("dateTime.iso8861")),
            Type::Base64(bytes) => Ok(bytes.clone()),
            Type::Struct { .. } => Err(err("struct")),
            Type::Array { .. } => Err(err("array")),
            #[cfg(feature = "nil")]
            Type::Nil => Err(err("nil")),
        }
    }
}

#[cfg(feature = "nil")]
impl<T> FromDXR for Option<T>
where
    T: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<Option<T>, DxrError> {
        if let Type::Nil = value.inner() {
            Ok(None)
        } else {
            Ok(Some(T::from_dxr(value)?))
        }
    }
}

impl<T> FromDXR for Vec<T>
where
    T: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<Vec<T>, DxrError> {
        let err = |t: &'static str| DxrError::wrong_type(t, "array");

        let values = match value.inner() {
            Type::Integer(_) => Err(err("i4")),
            #[cfg(feature = "i8")]
            Type::Long(_) => Err(err("i8")),
            Type::Boolean(_) => Err(err("boolean")),
            Type::String(_) => Err(err("string")),
            Type::Double(_) => Err(err("double")),
            Type::DateTime(_) => Err(err("dateTime.iso8861")),
            Type::Base64(_) => Err(err("base64")),
            Type::Struct { .. } => Err(err("struct")),
            Type::Array { data } => Ok(data.inner()),
            #[cfg(feature = "nil")]
            Type::Nil => Err(err("nil")),
        };

        values?.iter().map(|value| T::from_dxr(value)).collect()
    }
}

impl<T> FromDXR for HashMap<String, T>
where
    T: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<HashMap<String, T>, DxrError> {
        let err = |t: &'static str| DxrError::wrong_type(t, "array");

        let values = match value.inner() {
            Type::Integer(_) => Err(err("i4")),
            #[cfg(feature = "i8")]
            Type::Long(_) => Err(err("i8")),
            Type::Boolean(_) => Err(err("boolean")),
            Type::String(_) => Err(err("string")),
            Type::Double(_) => Err(err("double")),
            Type::DateTime(_) => Err(err("dateTime.iso8861")),
            Type::Base64(_) => Err(err("base64")),
            Type::Struct { members } => Ok(members),
            Type::Array { .. } => Err(err("array")),
            #[cfg(feature = "nil")]
            Type::Nil => Err(err("nil")),
        };

        values?
            .iter()
            .map(|v| {
                let name = v.name().to_string();
                match T::from_dxr(v.inner()) {
                    Ok(value) => Ok((name, value)),
                    Err(error) => Err(error),
                }
            })
            .collect()
    }
}
