use chrono::{NaiveDateTime, Utc};

use crate::traits::{TryFromValue, TryToValue};
use crate::values::Value;

#[test]
fn to_i32() {
    let value = 42i32;
    let expected = Value::i4(42);

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_i32() {
    let value = Value::i4(42);
    let expected = 42i32;

    assert_eq!(i32::try_from_value(&value).unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn to_i64() {
    let value = 42i64;
    let expected = Value::i8(42);

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn from_i64() {
    let value = Value::i8(42);
    let expected = 42i64;

    assert_eq!(i64::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_boolean() {
    let value = true;
    let expected = Value::boolean(true);

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_boolean() {
    let value = Value::boolean(false);
    let expected = false;

    assert_eq!(bool::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_string() {
    let value = "Hello, World!";
    let expected = Value::string("Hello, World!");

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_string() {
    let value = Value::string("Hello, World!");
    let expected = "Hello, World!";

    assert_eq!(String::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_double() {
    let value = 1.5f64;
    let expected = Value::double(1.5);

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_double() {
    let value = Value::double(1.5);
    let expected = 1.5f64;

    assert_eq!(f64::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_datetime() {
    let now = Utc::now().naive_utc();

    let value = now;
    let expected = Value::datetime(now);

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_datetime() {
    let now = Utc::now().naive_utc();

    let value = Value::datetime(now);
    let expected = now;

    assert_eq!(NaiveDateTime::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_base64() {
    let data = b"You can't read this!".to_vec();

    let value = data.clone();
    let expected = Value::base64(data);

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_base64() {
    let data = b"You can't read this!".to_vec();

    let value = Value::base64(data.clone());
    let expected = data;

    assert_eq!(<Vec<u8>>::try_from_value(&value).unwrap(), expected);
}
