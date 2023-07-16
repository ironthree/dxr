use std::rc::Rc;
use std::sync::Arc;

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
    let expected = Value::string(String::from("Hello, World!"));

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_string() {
    let value = Value::string(String::from("Hello, World!"));
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

#[test]
fn to_array() {
    let value = [1, 2, 3];
    let expected = vec![Value::i4(1), Value::i4(2), Value::i4(3)].try_to_value().unwrap();

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn to_box() {
    let value = Box::new(-12);
    let expected = Value::i4(-12);

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_box() {
    let value = Value::i4(-12);
    let expected = Box::new(-12);

    assert_eq!(<Box<i32>>::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_rc() {
    let value = Rc::new(-12);
    let expected = Value::i4(-12);

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_rc() {
    let value = Value::i4(-12);
    let expected = Rc::new(-12);

    assert_eq!(<Rc<i32>>::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_arc() {
    let value = Arc::new(-12);
    let expected = Value::i4(-12);

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_arc() {
    let value = Value::i4(-12);
    let expected = Arc::new(-12);

    assert_eq!(<Arc<i32>>::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_tuple_1() {
    let value = (true,);
    let expected = vec![Value::boolean(true)].try_to_value().unwrap();

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_tuple_1() {
    let value = vec![Value::boolean(true)].try_to_value().unwrap();
    let expected = (true,);

    assert_eq!(<(bool,)>::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_tuple_2() {
    let value = (true, 1);
    let expected = vec![Value::boolean(true), Value::i4(1)].try_to_value().unwrap();

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_tuple_2() {
    let value = vec![Value::boolean(true), Value::i4(1)].try_to_value().unwrap();
    let expected = (true, 1);

    assert_eq!(<(bool, i32)>::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_tuple_3() {
    let value = (true, 1, 2.5);
    let expected = vec![Value::boolean(true), Value::i4(1), Value::double(2.5)].try_to_value().unwrap();

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_tuple_3() {
    let value = vec![Value::boolean(true), Value::i4(1), Value::double(2.5)].try_to_value().unwrap();
    let expected = (true, 1, 2.5);

    assert_eq!(<(bool, i32, f64)>::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_tuple_4() {
    let value = (true, 1, 2.5, String::from("HELLO"));
    let expected = vec![Value::boolean(true), Value::i4(1), Value::double(2.5), Value::string(String::from("HELLO"))].try_to_value().unwrap();

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_tuple_4() {
    let value = vec![Value::boolean(true), Value::i4(1), Value::double(2.5), Value::string(String::from("HELLO"))].try_to_value().unwrap();
    let expected = (true, 1, 2.5, String::from("HELLO"));

    assert_eq!(<(bool, i32, f64, String)>::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_tuple_5() {
    let value = (true, 1, 2.5, String::from("HELLO"), -1);
    let expected = vec![Value::boolean(true), Value::i4(1), Value::double(2.5), Value::string(String::from("HELLO")), Value::i4(-1)].try_to_value().unwrap();

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_tuple_5() {
    let value = vec![Value::boolean(true), Value::i4(1), Value::double(2.5), Value::string(String::from("HELLO")), Value::i4(-1)].try_to_value().unwrap();
    let expected = (true, 1, 2.5, String::from("HELLO"), -1);

    assert_eq!(<(bool, i32, f64, String, i32)>::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_tuple_6() {
    let value = (true, 1, 2.5, String::from("HELLO"), -1, -1.5);
    let expected = vec![Value::boolean(true), Value::i4(1), Value::double(2.5), Value::string(String::from("HELLO")), Value::i4(-1), Value::double(-1.5)].try_to_value().unwrap();

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_tuple_6() {
    let value = vec![Value::boolean(true), Value::i4(1), Value::double(2.5), Value::string(String::from("HELLO")), Value::i4(-1), Value::double(-1.5)].try_to_value().unwrap();
    let expected = (true, 1, 2.5, String::from("HELLO"), -1, -1.5);

    assert_eq!(<(bool, i32, f64, String, i32, f64)>::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_tuple_7() {
    let value = (true, 1, 2.5, String::from("HELLO"), -1, -1.5, String::from("WORLD"));
    let expected = vec![Value::boolean(true), Value::i4(1), Value::double(2.5), Value::string(String::from("HELLO")), Value::i4(-1), Value::double(-1.5), Value::string(String::from("WORLD"))].try_to_value().unwrap();

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_tuple_7() {
    let value = vec![Value::boolean(true), Value::i4(1), Value::double(2.5), Value::string(String::from("HELLO")), Value::i4(-1), Value::double(-1.5), Value::string(String::from("WORLD"))].try_to_value().unwrap();
    let expected = (true, 1, 2.5, String::from("HELLO"), -1, -1.5, String::from("WORLD"));

    assert_eq!(<(bool, i32, f64, String, i32, f64, String)>::try_from_value(&value).unwrap(), expected);
}

#[test]
fn to_tuple_8() {
    let value = (true, 1, 2.5, String::from("HELLO"), -1, -1.5, String::from("WORLD"), false);
    let expected = vec![Value::boolean(true), Value::i4(1), Value::double(2.5), Value::string(String::from("HELLO")), Value::i4(-1), Value::double(-1.5), Value::string(String::from("WORLD")), Value::boolean(false)].try_to_value().unwrap();

    assert_eq!(value.try_to_value().unwrap(), expected);
}

#[test]
fn from_tuple_8() {
    let value = vec![Value::boolean(true), Value::i4(1), Value::double(2.5), Value::string(String::from("HELLO")), Value::i4(-1), Value::double(-1.5), Value::string(String::from("WORLD")), Value::boolean(false)].try_to_value().unwrap();
    let expected = (true, 1, 2.5, String::from("HELLO"), -1, -1.5, String::from("WORLD"), false);

    assert_eq!(<(bool, i32, f64, String, i32, f64, String, bool)>::try_from_value(&value).unwrap(), expected);
}
