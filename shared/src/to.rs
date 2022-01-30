use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::{Array, Member, Struct, ToValue, Value};

impl ToValue<Value> for Value {
    fn to_value(value: &Value) -> Value {
        value.clone()
    }
}

impl ToValue<i32> for i32 {
    fn to_value(value: &i32) -> Value {
        Value::i4(*value)
    }
}

#[cfg(feature = "i8")]
impl ToValue<i64> for i64 {
    fn to_value(value: &i64) -> Value {
        Value::i8(*value)
    }
}

impl ToValue<bool> for bool {
    fn to_value(value: &bool) -> Value {
        Value::boolean(*value)
    }
}

impl ToValue<String> for String {
    fn to_value(value: &String) -> Value {
        Value::string(value.clone())
    }
}

impl ToValue<f64> for f64 {
    fn to_value(value: &f64) -> Value {
        Value::double(*value)
    }
}

impl ToValue<DateTime<Utc>> for DateTime<Utc> {
    fn to_value(value: &DateTime<Utc>) -> Value {
        Value::datetime(*value)
    }
}

impl ToValue<Vec<u8>> for Vec<u8> {
    fn to_value(value: &Vec<u8>) -> Value {
        Value::base64(value.clone())
    }
}

impl<T> ToValue<Option<T>> for Option<T>
where
    T: ToValue<T>,
{
    fn to_value(value: &Option<T>) -> Value {
        if let Some(value) = value {
            T::to_value(value)
        } else {
            Value::nil()
        }
    }
}

impl<T> ToValue<Vec<T>> for Vec<T>
where
    T: ToValue<T>,
{
    fn to_value(value: &Vec<T>) -> Value {
        let values = value.iter().map(|value| T::to_value(value)).collect::<Vec<Value>>();

        Value::array(Array::new(values))
    }
}

impl<T> ToValue<HashMap<String, T>> for HashMap<String, T>
where
    T: ToValue<T>,
{
    fn to_value(value: &HashMap<String, T>) -> Value {
        let members = value
            .iter()
            .map(|(k, v)| Member::new(k.to_owned(), T::to_value(v)))
            .collect::<Vec<Member>>();

        Value::structure(Struct::new(members))
    }
}
