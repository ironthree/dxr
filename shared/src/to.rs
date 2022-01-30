use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::{Array, Member, Struct, ToDXR, Value};

impl ToDXR<Value> for Value {
    fn to_dxr(value: &Value) -> Value {
        value.clone()
    }
}

impl ToDXR<i32> for i32 {
    fn to_dxr(value: &i32) -> Value {
        Value::i4(*value)
    }
}

#[cfg(feature = "i8")]
impl ToDXR<i64> for i64 {
    fn to_dxr(value: &i64) -> Value {
        Value::i8(*value)
    }
}

impl ToDXR<bool> for bool {
    fn to_dxr(value: &bool) -> Value {
        Value::boolean(*value)
    }
}

impl ToDXR<String> for String {
    fn to_dxr(value: &String) -> Value {
        Value::string(value.clone())
    }
}

impl ToDXR<f64> for f64 {
    fn to_dxr(value: &f64) -> Value {
        Value::double(*value)
    }
}

impl ToDXR<DateTime<Utc>> for DateTime<Utc> {
    fn to_dxr(value: &DateTime<Utc>) -> Value {
        Value::datetime(*value)
    }
}

impl ToDXR<Vec<u8>> for Vec<u8> {
    fn to_dxr(value: &Vec<u8>) -> Value {
        Value::base64(value.clone())
    }
}

impl<T> ToDXR<Option<T>> for Option<T>
where
    T: ToDXR<T>,
{
    fn to_dxr(value: &Option<T>) -> Value {
        if let Some(value) = value {
            T::to_dxr(value)
        } else {
            Value::nil()
        }
    }
}

impl<T> ToDXR<Vec<T>> for Vec<T>
where
    T: ToDXR<T>,
{
    fn to_dxr(value: &Vec<T>) -> Value {
        let values = value.iter().map(|value| T::to_dxr(value)).collect::<Vec<Value>>();

        Value::array(Array::new(values))
    }
}

impl<T> ToDXR<HashMap<String, T>> for HashMap<String, T>
where
    T: ToDXR<T>,
{
    fn to_dxr(value: &HashMap<String, T>) -> Value {
        let members = value
            .iter()
            .map(|(k, v)| Member::new(k.to_owned(), T::to_dxr(v)))
            .collect::<Vec<Member>>();

        Value::structure(Struct::new(members))
    }
}
