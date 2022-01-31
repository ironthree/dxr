use std::collections::HashMap;

use chrono::{DateTime, Utc};
use quick_xml::escape::escape;

use crate::types::{Array, Member, Struct, Value};
use crate::{ToDXR, ValueError};

impl ToDXR for Value {
    fn to_dxr(&self) -> Result<Value, ValueError> {
        Ok(self.clone())
    }
}

impl ToDXR for i32 {
    fn to_dxr(&self) -> Result<Value, ValueError> {
        Ok(Value::i4(*self))
    }
}

#[cfg(feature = "i8")]
impl ToDXR for i64 {
    fn to_dxr(&self) -> Result<Value, ValueError> {
        Ok(Value::i8(*self))
    }
}

impl ToDXR for bool {
    fn to_dxr(&self) -> Result<Value, ValueError> {
        Ok(Value::boolean(*self))
    }
}

impl ToDXR for String {
    fn to_dxr(&self) -> Result<Value, ValueError> {
        let string =
            String::from_utf8(escape(self.trim().as_bytes()).to_vec()).map_err(|_| ValueError::InvalidContents)?;
        Ok(Value::string(string))
    }
}

impl ToDXR for f64 {
    fn to_dxr(&self) -> Result<Value, ValueError> {
        Ok(Value::double(*self))
    }
}

impl ToDXR for DateTime<Utc> {
    fn to_dxr(&self) -> Result<Value, ValueError> {
        Ok(Value::datetime(*self))
    }
}

impl ToDXR for Vec<u8> {
    fn to_dxr(&self) -> Result<Value, ValueError> {
        Ok(Value::base64(self.clone()))
    }
}

impl<T> ToDXR for Option<T>
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, ValueError> {
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
    fn to_dxr(&self) -> Result<Value, ValueError> {
        let values = self
            .iter()
            .map(|value| T::to_dxr(value))
            .collect::<Result<Vec<Value>, ValueError>>();

        Ok(Value::array(Array::new(values?)))
    }
}

impl<T> ToDXR for HashMap<String, T>
where
    T: ToDXR,
{
    fn to_dxr(&self) -> Result<Value, ValueError> {
        let members = self
            .iter()
            .map(|(k, v)| T::to_dxr(v).map(|v| Member::new(k.to_owned(), v)))
            .collect::<Result<Vec<Member>, ValueError>>();

        Ok(Value::structure(Struct::new(members?)))
    }
}
