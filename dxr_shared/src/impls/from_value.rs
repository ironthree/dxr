use std::borrow::Cow;
use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::error::DxrError;
use crate::traits::TryFromValue;
use crate::values::{Type, Value};

use super::utils::*;

impl TryFromValue for Value {
    fn try_from_value(value: &Value) -> Result<Value, DxrError> {
        Ok(value.clone())
    }
}

impl TryFromValue for i32 {
    fn try_from_value(value: &Value) -> Result<i32, DxrError> {
        match value.inner() {
            Type::Integer(int) => Ok(*int),
            t => Err(DxrError::wrong_type(t.name(), "i4")),
        }
    }
}

#[cfg(feature = "i8")]
#[cfg_attr(docsrs, doc(cfg(feature = "i8")))]
impl TryFromValue for i64 {
    fn try_from_value(value: &Value) -> Result<i64, DxrError> {
        match value.inner() {
            Type::Long(long) => Ok(*long),
            t => Err(DxrError::wrong_type(t.name(), "i8")),
        }
    }
}

impl TryFromValue for bool {
    fn try_from_value(value: &Value) -> Result<bool, DxrError> {
        match value.inner() {
            Type::Boolean(boo) => Ok(*boo),
            t => Err(DxrError::wrong_type(t.name(), "boolean")),
        }
    }
}

impl TryFromValue for String {
    fn try_from_value(value: &Value) -> Result<String, DxrError> {
        match value.inner() {
            Type::String(string) => Value::string_unescape(string),
            t => Err(DxrError::wrong_type(t.name(), "string")),
        }
    }
}

impl TryFromValue for f64 {
    fn try_from_value(value: &Value) -> Result<f64, DxrError> {
        match value.inner() {
            Type::Double(double) => Ok(*double),
            t => Err(DxrError::wrong_type(t.name(), "double")),
        }
    }
}

impl TryFromValue for DateTime<Utc> {
    fn try_from_value(value: &Value) -> Result<DateTime<Utc>, DxrError> {
        match value.inner() {
            Type::DateTime(date) => Ok(*date),
            t => Err(DxrError::wrong_type(t.name(), "dateTime.iso8861")),
        }
    }
}

impl TryFromValue for Vec<u8> {
    fn try_from_value(value: &Value) -> Result<Vec<u8>, DxrError> {
        match value.inner() {
            Type::Base64(bytes) => Ok(bytes.clone()),
            t => Err(DxrError::wrong_type(t.name(), "base64")),
        }
    }
}

#[cfg(feature = "nil")]
#[cfg_attr(docsrs, doc(cfg(feature = "nil")))]
impl<T> TryFromValue for Option<T>
where
    T: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<Option<T>, DxrError> {
        if let Type::Nil = value.inner() {
            Ok(None)
        } else {
            Ok(Some(T::try_from_value(value)?))
        }
    }
}

impl<T> TryFromValue for Cow<'_, T>
where
    T: TryFromValue + Clone,
{
    fn try_from_value(value: &Value) -> Result<Self, DxrError> {
        Ok(Cow::Owned(T::try_from_value(value)?))
    }
}

impl<T> TryFromValue for Box<T>
where
    T: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<Self, DxrError> {
        Ok(Box::new(T::try_from_value(value)?))
    }
}

impl<T> TryFromValue for Vec<T>
where
    T: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<Vec<T>, DxrError> {
        let values = match value.inner() {
            Type::Array { data } => Ok(data.inner()),
            t => Err(DxrError::wrong_type(t.name(), "array")),
        };

        values?.iter().map(|value| T::try_from_value(value)).collect()
    }
}

impl<T, const N: usize> TryFromValue for [T; N]
where
    T: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<Self, DxrError> {
        let values = match value.inner() {
            Type::Array { data } => Ok(data.inner()),
            t => Err(DxrError::wrong_type(t.name(), "array")),
        }?;

        let mapped: Vec<T> = values
            .iter()
            .map(|value| T::try_from_value(value))
            .collect::<Result<Vec<T>, DxrError>>()?;
        let len = mapped.len();

        mapped.try_into().map_err(|_| DxrError::parameter_mismatch(len, N))
    }
}

impl<T> TryFromValue for HashMap<String, T>
where
    T: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<HashMap<String, T>, DxrError> {
        let values = match value.inner() {
            Type::Struct { members } => Ok(members),
            t => Err(DxrError::wrong_type(t.name(), "struct")),
        };

        values?
            .iter()
            .map(|v| {
                let name = v.name().to_string();
                match T::try_from_value(v.inner()) {
                    Ok(value) => Ok((name, value)),
                    Err(error) => Err(error),
                }
            })
            .collect()
    }
}

// some implementations for exact numbers of values (with possibly different types)
impl TryFromValue for () {
    fn try_from_value(value: &Value) -> Result<Self, DxrError> {
        match value.inner() {
            Type::Array { data } => {
                let values = data.inner();

                match values.len() {
                    0 => Ok(()),
                    n => Err(DxrError::parameter_mismatch(n, 0)),
                }
            },
            #[cfg(feature = "nil")]
            Type::Nil => Ok(()),
            other => Err(DxrError::wrong_type(other.name(), "array | nil")),
        }
    }
}

impl<T> TryFromValue for (T,)
where
    T: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();
            values_to_tuple_1(values)
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B> TryFromValue for (A, B)
where
    A: TryFromValue,
    B: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();
            values_to_tuple_2(values)
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B, C> TryFromValue for (A, B, C)
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();
            values_to_tuple_3(values)
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B, C, D> TryFromValue for (A, B, C, D)
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();
            values_to_tuple_4(values)
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B, C, D, E> TryFromValue for (A, B, C, D, E)
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
    E: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();
            values_to_tuple_5(values)
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B, C, D, E, F> TryFromValue for (A, B, C, D, E, F)
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
    E: TryFromValue,
    F: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();
            values_to_tuple_6(values)
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B, C, D, E, F, G> TryFromValue for (A, B, C, D, E, F, G)
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
    E: TryFromValue,
    F: TryFromValue,
    G: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();
            values_to_tuple_7(values)
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

impl<A, B, C, D, E, F, G, H> TryFromValue for (A, B, C, D, E, F, G, H)
where
    A: TryFromValue,
    B: TryFromValue,
    C: TryFromValue,
    D: TryFromValue,
    E: TryFromValue,
    F: TryFromValue,
    G: TryFromValue,
    H: TryFromValue,
{
    fn try_from_value(value: &Value) -> Result<Self, DxrError> {
        if let Type::Array { data } = value.inner() {
            let values = data.inner();
            values_to_tuple_8(values)
        } else {
            Err(DxrError::wrong_type(value.inner().name(), "array"))
        }
    }
}

// if needed, implementations for more arguments can be implemented
