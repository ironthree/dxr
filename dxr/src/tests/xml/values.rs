use chrono::{SubsecRound, Utc};
use quick_xml::{de::from_str, se::to_string};

use crate::values::{Value, XML_RPC_DATE_FORMAT};

#[test]
fn to_i4() {
    let value = Value::i4(-12);
    let expected = "<value><i4>-12</i4></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_i4() {
    let value = "<value><i4>-12</i4></value>";
    let expected = Value::i4(-12);

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn from_int() {
    let value = "<value><int>-12</int></value>";
    let expected = Value::i4(-12);

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn to_i8() {
    let value = Value::i8(-12);
    let expected = "<value><i8>-12</i8></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn from_i8() {
    let value = "<value><i8>-12</i8></value>";
    let expected = Value::i8(-12);

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn to_boolean() {
    let value = Value::boolean(true);
    let expected = "<value><boolean>1</boolean></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_boolean() {
    let value = "<value><boolean>1</boolean></value>";
    let expected = Value::boolean(true);

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn to_str() {
    let value = Value::string(String::from("Hello, World!"));
    let expected = "<value><string>Hello, World!</string></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_string() {
    let value = "<value><string>Hello, World!</string></value>";
    let expected = Value::string(String::from("Hello, World!"));

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn to_str_with_escape() {
    let value = Value::string(String::from("a&b"));
    let expected = "<value><string>a&amp;b</string></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_string_with_escape() {
    let value = "<value><string>a&amp;b</string></value>";
    let expected = Value::string(String::from("a&b"));

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn from_untyped_string() {
    let value = "<value>Hello, World!</value>";
    let expected = Value::string(String::from("Hello, World!"));

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn from_untyped_empty_string() {
    let value = "<value></value>";
    let expected = Value::string(String::new());

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn from_untyped_empty_string_self_closing() {
    let value = "<value />";
    let expected = Value::string(String::new());

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn from_untyped_string_with_escape() {
    let value = "<value>a&amp;b</value>";
    let expected = Value::string(String::from("a&b"));

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn to_double() {
    let value = Value::double(1.5);
    let expected = "<value><double>1.5</double></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_double() {
    let value = "<value><double>1.5</double></value>";
    let expected = Value::double(1.5);

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn to_datetime() {
    let datetime = Utc::now().naive_utc();
    let datetime_str = datetime.format(XML_RPC_DATE_FORMAT).to_string();

    let value = Value::datetime(datetime);
    let expected = format!("<value><dateTime.iso8601>{datetime_str}</dateTime.iso8601></value>");

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_datetime() {
    let datetime = Utc::now().round_subsecs(0).naive_utc();
    let datetime_str = datetime.format(XML_RPC_DATE_FORMAT).to_string();

    let value = format!("<value><dateTime.iso8601>{datetime_str}</dateTime.iso8601></value>");
    let expected = Value::datetime(datetime);

    assert_eq!(from_str::<Value>(&value).unwrap(), expected);
}

#[test]
fn to_base64() {
    let contents = b"you can't read this!";
    let encoded = base64::encode(contents);

    let value = Value::base64(contents.to_vec());
    let expected = format!("<value><base64>{encoded}</base64></value>");

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_base64() {
    let contents = b"you can't read this!";
    let encoded = base64::encode(contents);

    let value = format!("<value><base64>{encoded}</base64></value>");
    let expected = Value::base64(contents.to_vec());

    assert_eq!(from_str::<Value>(&value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn to_nil() {
    let value = Value::nil();
    let expected = "<value><nil/></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn from_nil() {
    let value = "<value><nil/></value>";
    let expected = Value::nil();

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}
