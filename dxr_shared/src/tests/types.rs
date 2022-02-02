use chrono::{SubsecRound, Utc};
use quick_xml::{de::from_str, se::to_string};

use crate::types::*;
use crate::XML_RPC_DATE_FORMAT;

#[test]
fn to_i4() {
    let value = Type::Integer(-12);
    let expected = "<i4>-12</i4>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_i4() {
    let value = "<i4>-12</i4>";
    let expected = Type::Integer(-12);

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[test]
fn from_int() {
    let value = "<int>-12</int>";
    let expected = Type::Integer(-12);

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn to_i8() {
    let value = Type::Long(-12);
    let expected = "<i8>-12</i8>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn from_i8() {
    let value = "<i8>-12</i8>";
    let expected = Type::Long(-12);

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[test]
fn to_boolean() {
    let value = Type::Boolean(false);
    let expected = "<boolean>0</boolean>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_boolean() {
    let value = "<boolean>0</boolean>";
    let expected = Type::Boolean(false);

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[test]
fn to_str() {
    let value = Type::String(String::from("Hello, World!"));
    let expected = "<string>Hello, World!</string>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_string() {
    let value = "<string>Hello, World!</string>";
    let expected = Type::String(String::from("Hello, World!"));

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[test]
fn to_double() {
    let value = Type::Double(1.5);
    let expected = "<double>1.5</double>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_double() {
    let value = "<double>1.5</double>";
    let expected = Type::Double(1.5);

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[test]
fn to_datetime() {
    let datetime = Utc::now();
    let datetime_str = datetime.format(XML_RPC_DATE_FORMAT).to_string();

    let value = Type::DateTime(datetime);
    let expected = format!("<dateTime.iso8601>{}</dateTime.iso8601>", datetime_str);

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_datetime() {
    let datetime = Utc::now().round_subsecs(0);
    let datetime_str = datetime.format(XML_RPC_DATE_FORMAT).to_string();

    let value = format!("<dateTime.iso8601>{}</dateTime.iso8601>", datetime_str);
    let expected = Type::DateTime(datetime);

    assert_eq!(from_str::<Type>(&value).unwrap(), expected);
}

#[test]
fn to_base64() {
    let contents = b"you can't read this!";
    let encoded = base64::encode(contents);

    let value = Type::Base64(contents.to_vec());
    let expected = format!("<base64>{}</base64>", encoded);

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_base64() {
    let contents = b"you can't read this!";
    let encoded = base64::encode(contents);

    let value = format!("<base64>{}</base64>", encoded);
    let expected = Type::Base64(contents.to_vec());

    assert_eq!(from_str::<Type>(&value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn to_nil() {
    let value = Type::Nil;
    let expected = "<nil/>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn from_nil() {
    let value = "<nil/>";
    let expected = Type::Nil;

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}
