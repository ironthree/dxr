use std::collections::HashMap;

use chrono::{DateTime, Utc};
use quick_xml::escape::escape;

use crate::types::{Array, Member, Struct, Value};
use crate::{DxrError, ToDXR};

impl ToDXR for Value {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(self.clone())
    }
}

impl ToDXR for &Value {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::clone(self))
    }
}

impl ToDXR for i32 {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::i4(*self))
    }
}

#[cfg(feature = "i8")]
impl ToDXR for i64 {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::i8(*self))
    }
}

impl ToDXR for bool {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::boolean(*self))
    }
}

impl ToDXR for String {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        ToDXR::to_dxr(&self.as_str())
    }
}

impl ToDXR for &str {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let string = String::from_utf8(escape(self.trim().as_bytes()).to_vec())
            .map_err(|error| DxrError::invalid_data(error.to_string()))?;
        Ok(Value::string(string))
    }
}

impl ToDXR for f64 {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::double(*self))
    }
}

impl ToDXR for DateTime<Utc> {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::datetime(*self))
    }
}

impl ToDXR for Vec<u8> {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        ToDXR::to_dxr(&self.as_slice())
    }
}

impl<const N: usize> ToDXR for [u8; N] {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        ToDXR::to_dxr(&self.as_slice())
    }
}

impl ToDXR for &[u8] {
    fn to_dxr(&self) -> Result<Value, DxrError> {
        Ok(Value::base64(self.to_vec()))
    }
}

impl<T> ToDXR for Option<T>
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        if let Some(value) = self {
            T::to_dxr(value)
        } else {
            Ok(Value::nil())
        }
    }
}

impl<T> ToDXR for &Option<T>
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        if let Some(value) = self {
            T::to_dxr(value)
        } else {
            Ok(Value::nil())
        }
    }
}

impl<T> ToDXR for Vec<T>
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        ToDXR::to_dxr(&self.as_slice())
    }
}

impl<T, const N: usize> ToDXR for [T; N]
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        ToDXR::to_dxr(&self.as_slice())
    }
}

impl<T> ToDXR for &[T]
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let values = self
            .iter()
            .map(|value| T::to_dxr(value))
            .collect::<Result<Vec<Value>, DxrError>>();

        Ok(Value::array(Array::new(values?)))
    }
}

impl<T> ToDXR for HashMap<String, T>
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let members = self
            .iter()
            .map(|(k, v)| T::to_dxr(v).map(|v| Member::new(k.to_owned(), v)))
            .collect::<Result<Vec<Member>, DxrError>>();

        Ok(Value::structure(Struct::new(members?)))
    }
}

impl<T> ToDXR for HashMap<&str, T>
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, DxrError> {
        let members = self
            .iter()
            .map(|(k, v)| T::to_dxr(v).map(|v| Member::new((*k).to_owned(), v)))
            .collect::<Result<Vec<Member>, DxrError>>();

        Ok(Value::structure(Struct::new(members?)))
    }
}
