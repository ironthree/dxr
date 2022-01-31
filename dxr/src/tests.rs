#![allow(clippy::unwrap_used)]

use super::chrono::{DateTime, Utc};

use super::types::{Array, Member, Struct, Value};
use super::ToDXR;

#[test]
fn to_value_i32() {
    let value = 42i32;
    let expected = Value::i4(42);

    assert_eq!(i32::to_dxr(&value).unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn to_value_i64() {
    let value = 42i64;
    let expected = Value::i8(42);

    assert_eq!(i64::to_dxr(&value).unwrap(), expected);
}

#[test]
fn to_value_boolean() {
    let value = true;
    let expected = Value::boolean(true);

    assert_eq!(bool::to_dxr(&value).unwrap(), expected);
}

#[test]
fn to_value_string() {
    let value = String::from("Hello, World!");
    let expected = Value::string(String::from("Hello, World!"));

    assert_eq!(String::to_dxr(&value).unwrap(), expected);
}

#[test]
fn to_value_double() {
    let value = 1.5f64;
    let expected = Value::double(1.5);

    assert_eq!(f64::to_dxr(&value).unwrap(), expected);
}

#[test]
fn to_value_datetime() {
    let now = Utc::now();

    let value = now;
    let expected = Value::datetime(now);

    assert_eq!(<DateTime<Utc>>::to_dxr(&value).unwrap(), expected);
}

#[test]
fn to_value_base64() {
    let data = b"You can't read this!".to_vec();

    let value = data.clone();
    let expected = Value::base64(data);

    assert_eq!(<Vec<u8>>::to_dxr(&value).unwrap(), expected);
}

#[test]
fn to_value_struct_empty() {
    #[derive(ToDXR)]
    struct Test {}

    let value = Test {};
    let expected = Value::structure(Struct::new(vec![]));

    assert_eq!(Test::to_dxr(&value).unwrap(), expected);
}

#[test]
fn to_value_struct() {
    #[derive(ToDXR)]
    struct Test {
        id: i32,
    }

    let value = Test { id: 42 };
    let expected = Value::structure(Struct::new(vec![Member::new(String::from("id"), Value::i4(42))]));

    assert_eq!(Test::to_dxr(&value).unwrap(), expected);
}

#[test]
fn to_value_array() {
    let value = vec![-12, 42];
    let expected = Value::array(Array::new(vec![Value::i4(-12), Value::i4(42)]));

    assert_eq!(<Vec<i32>>::to_dxr(&value).unwrap(), expected);
}

#[test]
fn to_value_some() {
    let value = Some(42i32);
    let expected = Value::i4(42);

    assert_eq!(<Option<i32>>::to_dxr(&value).unwrap(), expected);
}

#[test]
fn to_value_none() {
    let value: Option<i32> = None;
    let expected = Value::nil();

    assert_eq!(<Option<i32>>::to_dxr(&value).unwrap(), expected);
}
