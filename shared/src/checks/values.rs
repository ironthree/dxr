use quick_xml::escape::escape;
use quick_xml::{de::from_str, se::to_string};
use quickcheck_macros::quickcheck;

use crate::Value;

#[quickcheck]
fn to_from_i4(int: i32) -> bool {
    let value = Value::i4(int);

    value == from_str::<Value>(&to_string(&value).unwrap()).unwrap()
}

#[quickcheck]
fn from_to_i4(int: i32) -> bool {
    let value = format!("<value><i4>{}</i4></value>", int);

    value == to_string(&from_str::<Value>(&value).unwrap()).unwrap()
}

#[cfg(feature = "i8")]
#[quickcheck]
fn to_from_i8(long: i64) -> bool {
    let value = Value::i8(long);

    value == from_str::<Value>(&to_string(&value).unwrap()).unwrap()
}

#[cfg(feature = "i8")]
#[quickcheck]
fn from_to_i8(long: i64) -> bool {
    let value = format!("<value><i8>{}</i8></value>", long);

    value == to_string(&from_str::<Value>(&value).unwrap()).unwrap()
}

#[quickcheck]
fn to_from_boolean(boolean: bool) -> bool {
    let value = Value::boolean(boolean);

    value == from_str::<Value>(&to_string(&value).unwrap()).unwrap()
}

#[quickcheck]
fn from_to_boolean(boolean: bool) -> bool {
    let value = format!("<value><boolean>{}</boolean></value>", boolean as i32);

    value == to_string(&from_str::<Value>(&value).unwrap()).unwrap()
}

#[quickcheck]
fn to_from_string(string: String) -> bool {
    // This creates a new <string> value on a code path that does no XML escaping,
    // so the string needs to be trimmed and XML-escaped first.
    let string = String::from_utf8(escape(string.trim().as_bytes()).to_vec()).unwrap();
    let value = Value::string(string);

    value == from_str::<Value>(&to_string(&value).unwrap()).unwrap()
}

#[quickcheck]
fn from_to_string(string: String) -> bool {
    // This creates a new <string> value on a code path that does no XML escaping,
    // so the string needs to be trimmed and XML-escaped first.
    let string = String::from_utf8(escape(string.trim().as_bytes()).to_vec()).unwrap();
    let value = format!("<value><string>{}</string></value>", string);

    value == to_string(&from_str::<Value>(&value).unwrap()).unwrap()
}

#[quickcheck]
fn to_from_double(double: f64) -> bool {
    if double.is_nan() {
        return true;
    }

    let value = Value::double(double);

    value == from_str::<Value>(&to_string(&value).unwrap()).unwrap()
}

#[quickcheck]
fn from_to_double(double: f64) -> bool {
    if double.is_nan() {
        return true;
    }

    let value = format!("<value><double>{}</double></value>", double);

    value == to_string(&from_str::<Value>(&value).unwrap()).unwrap()
}

#[quickcheck]
fn to_from_base64(bytes: Vec<u8>) -> bool {
    let value = Value::base64(bytes);

    value == from_str::<Value>(&to_string(&value).unwrap()).unwrap()
}

#[quickcheck]
fn from_to_base64(bytes: Vec<u8>) -> bool {
    let value = format!("<value><base64>{}</base64></value>", base64::encode(bytes));

    value == to_string(&from_str::<Value>(&value).unwrap()).unwrap()
}
