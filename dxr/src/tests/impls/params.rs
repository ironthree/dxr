use chrono::{NaiveDateTime, SubsecRound, Utc};

use crate::{TryFromParams, TryToParams, Value};

#[test]
fn to_value() {
    let value = vec![Value::i4(42)];
    let expected = Value::i4(42);

    assert_eq!(Value::try_from_params(&value).unwrap(), expected);
}

#[test]
fn from_value() {
    let value = Value::i4(42);
    let expected = vec![Value::i4(42)];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn from_value_ref() {
    let value = Value::i4(42);
    let expected = vec![Value::i4(42)];

    assert_eq!(<&Value>::try_to_params(&&value).unwrap(), expected);
}

#[test]
fn to_i4() {
    let value = vec![Value::i4(-12)];
    let expected = -12i32;

    assert_eq!(i32::try_from_params(&value).unwrap(), expected);
}

#[test]
fn from_i4() {
    let value = -12i32;
    let expected = vec![Value::i4(-12)];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn to_i8() {
    let value = vec![Value::i8(-12)];
    let expected = -12i64;

    assert_eq!(i64::try_from_params(&value).unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn from_i8() {
    let value = -12i64;
    let expected = vec![Value::i8(-12)];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_boolean() {
    let value = vec![Value::boolean(true)];
    let expected = true;

    assert_eq!(bool::try_from_params(&value).unwrap(), expected);
}

#[test]
fn from_boolean() {
    let value = false;
    let expected = vec![Value::boolean(false)];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_str() {
    let expected = "HELLO";
    let value = vec![Value::string(String::from(expected))];

    assert_eq!(String::try_from_params(&value).unwrap(), expected);
}

#[test]
fn from_str() {
    let value = "WORLD";
    let expected = vec![Value::string(String::from(value))];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn from_string() {
    let value = String::from("WORLD");
    let expected = vec![Value::string(value.clone())];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_double() {
    let value = vec![Value::double(-2.5)];
    let expected = -2.5f64;

    assert_eq!(f64::try_from_params(&value).unwrap(), expected);
}

#[test]
fn from_double() {
    let value = -2.5f64;
    let expected = vec![Value::double(-2.5)];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_datetime() {
    let expected = Utc::now().round_subsecs(0).naive_utc();
    let value = vec![Value::datetime(expected)];

    assert_eq!(NaiveDateTime::try_from_params(&value).unwrap(), expected);
}

#[test]
fn from_from_datetimedouble() {
    let value = Utc::now().round_subsecs(0).naive_utc();
    let expected = vec![Value::datetime(value)];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_base64() {
    let expected = b"you can't read this!".to_vec();
    let value = vec![Value::base64(expected.clone())];

    assert_eq!(<Vec<u8>>::try_from_params(&value).unwrap(), expected);
}

#[test]
fn from_base64() {
    let value = b"you can't read this!".to_vec();
    let expected = vec![Value::base64(value.clone())];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn from_base64_slice() {
    let value = b"you can't read this!";
    let expected = vec![Value::base64(value.to_vec())];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn from_base64_array() {
    let value = b"you can't read this!".to_owned();
    let expected = vec![Value::base64(value.to_vec())];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn to_nil_none() {
    let value = vec![Value::nil()];
    let expected = None;

    assert_eq!(<Option<i32>>::try_from_params(&value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn to_nil_missing() {
    let value: Vec<Value> = Vec::new();
    let expected = None;

    assert_eq!(<Option<i32>>::try_from_params(&value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn to_nil_some() {
    let value = vec![Value::i4(1)];
    let expected = Some(1);

    assert_eq!(<Option<i32>>::try_from_params(&value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn to_nil_fail() {
    let value = vec![Value::i4(1), Value::i4(2)];
    assert!(<Option<i32>>::try_from_params(&value)
        .unwrap_err()
        .is_parameter_mismatch());
}

#[cfg(feature = "nil")]
#[test]
fn from_nil_none() {
    let value: Option<NaiveDateTime> = None;
    let expected = vec![Value::nil()];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn from_nil_none_ref() {
    let value: Option<NaiveDateTime> = None;
    let expected = vec![Value::nil()];

    assert_eq!(<&Option<NaiveDateTime>>::try_to_params(&&value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn from_nil_some() {
    let dt = Utc::now().round_subsecs(0).naive_utc();
    let value: Option<NaiveDateTime> = Some(dt);
    let expected = vec![Value::datetime(dt)];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn from_nil_some_ref() {
    let dt = Utc::now().round_subsecs(0).naive_utc();
    let value: Option<NaiveDateTime> = Some(dt);
    let expected = vec![Value::datetime(dt)];

    assert_eq!(<&Option<NaiveDateTime>>::try_to_params(&&value).unwrap(), expected);
}

#[cfg(feature = "derive")]
#[test]
fn to_hashmap() {
    use crate::TryToValue;
    use std::collections::HashMap;

    #[derive(TryToValue)]
    struct Test {
        foo: i32,
    }

    let value = vec![Test { foo: -12 }.try_to_value().unwrap()];
    let mut expected = HashMap::new();
    expected.insert(String::from("foo"), -12);

    assert_eq!(HashMap::try_from_params(&value).unwrap(), expected);
}

#[cfg(feature = "derive")]
#[test]
fn from_hashmap() {
    use crate::TryToValue;
    use std::collections::HashMap;

    #[derive(TryToValue)]
    struct Test {
        foo: i32,
    }

    let mut value = HashMap::new();
    value.insert("foo", -12);
    let expected = vec![Test { foo: -12 }.try_to_value().unwrap()];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_vec() {
    let value = vec![Value::i4(8), Value::i4(16)];
    let expected = vec![8, 16];

    assert_eq!(<Vec<i32>>::try_from_params(&value).unwrap(), expected);
}

#[test]
fn from_vec() {
    let value = vec![8, 16];
    let expected = vec![Value::i4(8), Value::i4(16)];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_unit() {
    let value = Vec::new();
    <()>::try_from_params(&value).unwrap();
}

#[test]
fn to_unit_fail() {
    let value = vec![Value::i4(-1)];
    assert!(<()>::try_from_params(&value).unwrap_err().is_parameter_mismatch());
}

#[test]
fn from_unit() {
    let expected = Vec::new();
    assert_eq!(().try_to_params().unwrap(), expected);
}

#[test]
fn to_tuple_1() {
    let value = vec![Value::boolean(true)];
    let expected = (true,);

    assert_eq!(<(bool,)>::try_from_params(&value).unwrap(), expected);
}

#[test]
fn from_tuple_1() {
    let value = (true,);
    let expected = vec![Value::boolean(true)];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_tuple_2() {
    let value = vec![Value::boolean(true), Value::i4(1)];
    let expected = (true, 1);

    assert_eq!(<(bool, i32)>::try_from_params(&value).unwrap(), expected);
}

#[test]
fn from_tuple_2() {
    let value = (true, 1);
    let expected = vec![Value::boolean(true), Value::i4(1)];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_tuple_3() {
    let value = vec![Value::boolean(true), Value::i4(1), Value::double(2.5)];
    let expected = (true, 1, 2.5);

    assert_eq!(<(bool, i32, f64)>::try_from_params(&value).unwrap(), expected);
}

#[test]
fn from_tuple_3() {
    let value = (true, 1, 2.5);
    let expected = vec![Value::boolean(true), Value::i4(1), Value::double(2.5)];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_tuple_4() {
    let value = vec![
        Value::boolean(true),
        Value::i4(1),
        Value::double(2.5),
        Value::string(String::from("HELLO")),
    ];
    let expected = (true, 1, 2.5, String::from("HELLO"));

    assert_eq!(<(bool, i32, f64, String)>::try_from_params(&value).unwrap(), expected);
}

#[test]
fn from_tuple_4() {
    let value = (true, 1, 2.5, String::from("HELLO"));
    let expected = vec![
        Value::boolean(true),
        Value::i4(1),
        Value::double(2.5),
        Value::string(String::from("HELLO")),
    ];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_tuple_5() {
    let value = vec![
        Value::boolean(true),
        Value::i4(1),
        Value::double(2.5),
        Value::string(String::from("HELLO")),
        Value::boolean(false),
    ];
    let expected = (true, 1, 2.5, String::from("HELLO"), false);

    assert_eq!(
        <(bool, i32, f64, String, bool)>::try_from_params(&value).unwrap(),
        expected
    );
}

#[test]
fn from_tuple_5() {
    let value = (true, 1, 2.5, String::from("HELLO"), false);
    let expected = vec![
        Value::boolean(true),
        Value::i4(1),
        Value::double(2.5),
        Value::string(String::from("HELLO")),
        Value::boolean(false),
    ];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_tuple_6() {
    let value = vec![
        Value::boolean(true),
        Value::i4(1),
        Value::double(2.5),
        Value::string(String::from("HELLO")),
        Value::boolean(false),
        Value::i4(2),
    ];
    let expected = (true, 1, 2.5, String::from("HELLO"), false, 2);

    assert_eq!(
        <(bool, i32, f64, String, bool, i32)>::try_from_params(&value).unwrap(),
        expected
    );
}

#[test]
fn from_tuple_6() {
    let value = (true, 1, 2.5, String::from("HELLO"), false, 2);
    let expected = vec![
        Value::boolean(true),
        Value::i4(1),
        Value::double(2.5),
        Value::string(String::from("HELLO")),
        Value::boolean(false),
        Value::i4(2),
    ];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_tuple_7() {
    let value = vec![
        Value::boolean(true),
        Value::i4(1),
        Value::double(2.5),
        Value::string(String::from("HELLO")),
        Value::boolean(false),
        Value::i4(2),
        Value::double(-1.5),
    ];
    let expected = (true, 1, 2.5, String::from("HELLO"), false, 2, -1.5);

    assert_eq!(
        <(bool, i32, f64, String, bool, i32, f64)>::try_from_params(&value).unwrap(),
        expected
    );
}

#[test]
fn from_tuple_7() {
    let value = (true, 1, 2.5, String::from("HELLO"), false, 2, -1.5);
    let expected = vec![
        Value::boolean(true),
        Value::i4(1),
        Value::double(2.5),
        Value::string(String::from("HELLO")),
        Value::boolean(false),
        Value::i4(2),
        Value::double(-1.5),
    ];

    assert_eq!(value.try_to_params().unwrap(), expected);
}

#[test]
fn to_tuple_8() {
    let value = vec![
        Value::boolean(true),
        Value::i4(1),
        Value::double(2.5),
        Value::string(String::from("HELLO")),
        Value::boolean(false),
        Value::i4(2),
        Value::double(-1.5),
        Value::string(String::from("WORLD")),
    ];
    let expected = (
        true,
        1,
        2.5,
        String::from("HELLO"),
        false,
        2,
        -1.5,
        String::from("WORLD"),
    );

    assert_eq!(
        <(bool, i32, f64, String, bool, i32, f64, String)>::try_from_params(&value).unwrap(),
        expected
    );
}

#[test]
fn from_tuple_8() {
    let value = (
        true,
        1,
        2.5,
        String::from("HELLO"),
        false,
        2,
        -1.5,
        String::from("WORLD"),
    );
    let expected = vec![
        Value::boolean(true),
        Value::i4(1),
        Value::double(2.5),
        Value::string(String::from("HELLO")),
        Value::boolean(false),
        Value::i4(2),
        Value::double(-1.5),
        Value::string(String::from("WORLD")),
    ];

    assert_eq!(value.try_to_params().unwrap(), expected);
}
