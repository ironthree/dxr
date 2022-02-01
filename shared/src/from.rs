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

// some implementations for exact numbers of values (with possibly different types)
impl FromDXR for () {
    fn from_dxr(value: &Value) -> Result<Self, DxrError> {
        match value.inner() {
            Type::Array { data } => {
                let values = data.inner();

                match values.len() {
                    0 => Ok(()),
                    n => Err(DxrError::return_mismatch(n, 0)),
                }
            },
            Type::Nil => Ok(()),
            other => Err(DxrError::wrong_type(other.name(), "array | nil")),
        }
    }
}

impl<T> FromDXR for (T,)
where
    T: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();

            match values.len() {
                1 => {
                    let value = values.get(0).unwrap();

                    Ok((T::from_dxr(value)?,))
                },
                n => Err(DxrError::return_mismatch(n, 1)),
            }
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B> FromDXR for (A, B)
where
    A: FromDXR,
    B: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();

            match values.len() {
                2 => {
                    let v0 = values.get(0).unwrap();
                    let v1 = values.get(1).unwrap();

                    Ok((A::from_dxr(v0)?, B::from_dxr(v1)?))
                },
                n => Err(DxrError::return_mismatch(n, 2)),
            }
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B, C> FromDXR for (A, B, C)
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();

            match values.len() {
                3 => {
                    let v0 = values.get(0).unwrap();
                    let v1 = values.get(1).unwrap();
                    let v2 = values.get(2).unwrap();

                    Ok((A::from_dxr(v0)?, B::from_dxr(v1)?, C::from_dxr(v2)?))
                },
                n => Err(DxrError::return_mismatch(n, 3)),
            }
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B, C, D> FromDXR for (A, B, C, D)
where
    A: FromDXR,
    B: FromDXR,
    C: FromDXR,
    D: FromDXR,
{
    fn from_dxr(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();

            match values.len() {
                4 => {
                    let v0 = values.get(0).unwrap();
                    let v1 = values.get(1).unwrap();
                    let v2 = values.get(2).unwrap();
                    let v3 = values.get(3).unwrap();

                    Ok((A::from_dxr(v0)?, B::from_dxr(v1)?, C::from_dxr(v2)?, D::from_dxr(v3)?))
                },
                n => Err(DxrError::return_mismatch(n, 4)),
            }
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}
