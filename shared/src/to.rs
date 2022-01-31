use std::collections::HashMap;

use chrono::{DateTime, Utc};
use quick_xml::escape::escape;

use crate::types::{Array, Member, Struct, Value};
use crate::{ToDXR, ValueError};

impl ToDXR<Value> for Value {
    fn to_dxr(value: &Value) -> Result<Value, ValueError> {
        Ok(value.clone())
    }
}

impl ToDXR<i32> for i32 {
    fn to_dxr(value: &i32) -> Result<Value, ValueError> {
        Ok(Value::i4(*value))
    }
}

#[cfg(feature = "i8")]
impl ToDXR<i64> for i64 {
    fn to_dxr(value: &i64) -> Result<Value, ValueError> {
        Ok(Value::i8(*value))
    }
}

impl ToDXR<bool> for bool {
    fn to_dxr(value: &bool) -> Result<Value, ValueError> {
        Ok(Value::boolean(*value))
    }
}

impl ToDXR<String> for String {
    fn to_dxr(value: &String) -> Result<Value, ValueError> {
        let string =
            String::from_utf8(escape(value.trim().as_bytes()).to_vec()).map_err(|_| ValueError::InvalidContents)?;
        Ok(Value::string(string))
    }
}

impl ToDXR<f64> for f64 {
    fn to_dxr(value: &f64) -> Result<Value, ValueError> {
        Ok(Value::double(*value))
    }
}

impl ToDXR<DateTime<Utc>> for DateTime<Utc> {
    fn to_dxr(value: &DateTime<Utc>) -> Result<Value, ValueError> {
        Ok(Value::datetime(*value))
    }
}

impl ToDXR<Vec<u8>> for Vec<u8> {
    fn to_dxr(value: &Vec<u8>) -> Result<Value, ValueError> {
        Ok(Value::base64(value.clone()))
    }
}

impl<T> ToDXR<Option<T>> for Option<T>
where
    T: ToDXR<T>,
{
    fn to_dxr(value: &Option<T>) -> Result<Value, ValueError> {
        if let Some(value) = value {
            T::to_dxr(value)
        } else {
            Ok(Value::nil())
        }
    }
}

impl<T> ToDXR<Vec<T>> for Vec<T>
where
    T: ToDXR<T>,
{
    fn to_dxr(value: &Vec<T>) -> Result<Value, ValueError> {
        let values = value
            .iter()
            .map(|value| T::to_dxr(value))
            .collect::<Result<Vec<Value>, ValueError>>();

        Ok(Value::array(Array::new(values?)))
    }
}

impl<T> ToDXR<HashMap<String, T>> for HashMap<String, T>
where
    T: ToDXR<T>,
{
    fn to_dxr(value: &HashMap<String, T>) -> Result<Value, ValueError> {
        let members = value
            .iter()
            .map(|(k, v)| T::to_dxr(v).map(|v| Member::new(k.to_owned(), v)))
            .collect::<Result<Vec<Member>, ValueError>>();

        Ok(Value::structure(Struct::new(members?)))
    }
}
