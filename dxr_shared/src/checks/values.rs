use std::borrow::Cow;

use quick_xml::escape::escape;
use quick_xml::{de::from_str, se::to_string};
use quickcheck::TestResult;
use quickcheck_macros::quickcheck;

use crate::traits::{TryFromValue, TryToValue};
use crate::values::{Type, Value};

#[quickcheck]
fn to_from_i4(int: i32) -> bool {
    let value = Value::i4(int);

    value == from_str::<Value>(&to_string(&value).unwrap()).unwrap()
}

#[quickcheck]
fn from_to_i4(int: i32) -> bool {
    let value = format!("<value><i4>{int}</i4></value>");

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
    let value = format!("<value><i8>{long}</i8></value>");

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
    let string = escape(string.trim()).to_string();
    let value = Value::string(&string);

    value == from_str::<Value>(&to_string(&value).unwrap()).unwrap()
}

#[quickcheck]
fn from_to_string(string: String) -> bool {
    // This creates a new <string> value on a code path that does no XML escaping,
    // so the string needs to be trimmed and XML-escaped first.
    let string = escape(string.trim()).to_string();
    let value = format!("<value><string>{string}</string></value>");

    value == to_string(&from_str::<Value>(&value).unwrap()).unwrap()
}

#[quickcheck]
fn to_from_double(double: f64) -> TestResult {
    if double.is_nan() {
        return TestResult::discard();
    }

    let value = Value::double(double);

    TestResult::from_bool(value == from_str::<Value>(&to_string(&value).unwrap()).unwrap())
}

#[quickcheck]
fn from_to_double(double: f64) -> TestResult {
    if double.is_nan() {
        return TestResult::discard();
    }

    let value = format!("<value><double>{double}</double></value>");

    TestResult::from_bool(value == to_string(&from_str::<Value>(&value).unwrap()).unwrap())
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

#[cfg(feature = "nil")]
#[quickcheck]
fn roundtrip_option_some(a: i32) -> bool {
    let value = Some(a);
    <Option<i32>>::try_from_value(&value.try_to_value().unwrap()).unwrap() == value
}

#[quickcheck]
fn roundtrip_cow_string(string: String) -> bool {
    let expected: Cow<'_, String> = Cow::Owned(string.trim().to_owned());
    let value = <Cow<String>>::try_from_value(&TryToValue::try_to_value(&expected).unwrap()).unwrap();

    println!("Expected:");
    println!("{expected:#?}");
    println!("Value:");
    println!("{value:#?}");

    expected == value
}

#[quickcheck]
fn roundtrip_cow_i4(i4: i32) -> bool {
    let expected: Cow<'_, i32> = Cow::Owned(i4);
    <Cow<i32>>::try_from_value(&TryToValue::try_to_value(&expected).unwrap()).unwrap() == expected
}

#[quickcheck]
fn roundtrip_array(a: i32, b: i32) -> bool {
    let value = vec![a, b];
    <Vec<i32>>::try_from_value(&value.try_to_value().unwrap()).unwrap() == value
}

#[quickcheck]
fn roundtrip_string_escape_unescape(string: String) -> TestResult {
    // whitespace prefix and suffix are not preserved in XML
    let input = string.trim();

    let escaped = Value::string(input);

    let inner = match escaped.inner() {
        Type::String(s) => s,
        _ => return TestResult::error(format!("String got wrapped in the wrong Value type: {string}")),
    };

    let output = match Value::string_unescape(inner) {
        Ok(s) => s,
        Err(_) => return TestResult::error(format!("String failed to be un-escaped: {inner}")),
    };

    TestResult::from_bool(output == input)
}
