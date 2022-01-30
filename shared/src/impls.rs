use crate::{FromValue, Type, Value};
use chrono::{DateTime, Utc};

impl FromValue<i32> for i32 {
    fn from_value(value: &Value) -> Result<i32, ()> {
        match value.inner() {
            Type::Integer(int) => Ok(*int),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "i8")]
impl FromValue<i64> for i64 {
    fn from_value(value: &Value) -> Result<i64, ()> {
        match value.inner() {
            Type::Long(long) => Ok(*long),
            _ => Err(()),
        }
    }
}

impl FromValue<bool> for bool {
    fn from_value(value: &Value) -> Result<bool, ()> {
        match value.inner() {
            Type::Boolean(boo) => Ok(*boo),
            _ => Err(()),
        }
    }
}

impl FromValue<String> for String {
    fn from_value(value: &Value) -> Result<String, ()> {
        match value.inner() {
            Type::String(string) => Ok(string.clone()),
            _ => Err(()),
        }
    }
}

impl FromValue<f64> for f64 {
    fn from_value(value: &Value) -> Result<f64, ()> {
        match value.inner() {
            Type::Double(double) => Ok(*double),
            _ => Err(()),
        }
    }
}

impl FromValue<DateTime<Utc>> for DateTime<Utc> {
    fn from_value(value: &Value) -> Result<DateTime<Utc>, ()> {
        match value.inner() {
            Type::DateTime(date) => Ok(*date),
            _ => Err(()),
        }
    }
}

impl FromValue<Vec<u8>> for Vec<u8> {
    fn from_value(value: &Value) -> Result<Vec<u8>, ()> {
        match value.inner() {
            Type::Base64(bytes) => Ok(bytes.clone()),
            _ => Err(()),
        }
    }
}

#[cfg(feature = "nil")]
impl<T> FromValue<Option<T>> for Option<T>
where
    T: FromValue<T>,
{
    fn from_value(value: &Value) -> Result<Option<T>, ()> {
        if let Type::Nil = value.inner() {
            Ok(None)
        } else {
            Ok(Some(T::from_value(value)?))
        }
    }
}

impl<T> FromValue<Vec<T>> for Vec<T>
where
    T: FromValue<T>,
{
    fn from_value(value: &Value) -> Result<Vec<T>, ()> {
        let values = match value.inner() {
            Type::Array { data } => data.inner(),
            _ => return Err(()),
        };

        values.iter().map(|value| T::from_value(value)).collect()
    }
}
