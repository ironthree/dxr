use chrono::{DateTime, Utc};

use crate::traits::{FromDXR, ToDXR};
use crate::values::Value;

#[test]
fn to_i32() {
    let value = 42i32;
    let expected = Value::i4(42);

    assert_eq!(value.to_dxr().unwrap(), expected);
}

#[test]
fn from_i32() {
    let value = Value::i4(42);
    let expected = 42i32;

    assert_eq!(i32::from_dxr(&value).unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn to_i64() {
    let value = 42i64;
    let expected = Value::i8(42);

    assert_eq!(value.to_dxr().unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn from_i64() {
    let value = Value::i8(42);
    let expected = 42i64;

    assert_eq!(i64::from_dxr(&value).unwrap(), expected);
}

#[test]
fn to_boolean() {
    let value = true;
    let expected = Value::boolean(true);

    assert_eq!(value.to_dxr().unwrap(), expected);
}

#[test]
fn from_boolean() {
    let value = Value::boolean(false);
    let expected = false;

    assert_eq!(bool::from_dxr(&value).unwrap(), expected);
}

#[test]
fn to_string() {
    let value = "Hello, World!";
    let expected = Value::string_escape("Hello, World!").unwrap();

    assert_eq!(value.to_dxr().unwrap(), expected);
}

#[test]
fn from_string() {
    let value = Value::string_escape("Hello, World!").unwrap();
    let expected = "Hello, World!";

    assert_eq!(String::from_dxr(&value).unwrap(), expected);
}

#[test]
fn to_double() {
    let value = 1.5f64;
    let expected = Value::double(1.5);

    assert_eq!(value.to_dxr().unwrap(), expected);
}

#[test]
fn from_double() {
    let value = Value::double(1.5);
    let expected = 1.5f64;

    assert_eq!(f64::from_dxr(&value).unwrap(), expected);
}

#[test]
fn to_datetime() {
    let now = Utc::now();

    let value = now;
    let expected = Value::datetime(now);

    assert_eq!(value.to_dxr().unwrap(), expected);
}

#[test]
fn from_datetime() {
    let now = Utc::now();

    let value = Value::datetime(now);
    let expected = now;

    assert_eq!(DateTime::from_dxr(&value).unwrap(), expected);
}

#[test]
fn to_base64() {
    let data = b"You can't read this!".to_vec();

    let value = data.clone();
    let expected = Value::base64(data);

    assert_eq!(value.to_dxr().unwrap(), expected);
}

#[test]
fn from_base64() {
    let data = b"You can't read this!".to_vec();

    let value = Value::base64(data.clone());
    let expected = data;

    assert_eq!(<Vec<u8>>::from_dxr(&value).unwrap(), expected);
}
