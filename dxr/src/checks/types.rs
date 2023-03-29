use quick_xml::escape::escape;
use quick_xml::{de::from_str, se::to_string};
use quickcheck::TestResult;
use quickcheck_macros::quickcheck;

use crate::values::Type;

#[quickcheck]
fn to_from_i4(int: i32) -> bool {
    let value = Type::Integer(int);

    value == from_str::<Type>(&to_string(&value).unwrap()).unwrap()
}

#[quickcheck]
fn from_to_i4(int: i32) -> bool {
    let value = format!("<i4>{int}</i4>");

    value == to_string(&from_str::<Type>(&value).unwrap()).unwrap()
}

#[cfg(feature = "i8")]
#[quickcheck]
fn to_from_i8(long: i64) -> bool {
    let value = Type::Long(long);

    value == from_str::<Type>(&to_string(&value).unwrap()).unwrap()
}

#[cfg(feature = "i8")]
#[quickcheck]
fn from_to_i8(long: i64) -> bool {
    let value = format!("<i8>{long}</i8>");

    value == to_string(&from_str::<Type>(&value).unwrap()).unwrap()
}

#[quickcheck]
fn to_from_boolean(boolean: bool) -> bool {
    let value = Type::Boolean(boolean);

    value == from_str::<Type>(&to_string(&value).unwrap()).unwrap()
}

#[quickcheck]
fn from_to_boolean(boolean: bool) -> bool {
    let value = format!("<boolean>{}</boolean>", boolean as i32);

    value == to_string(&from_str::<Type>(&value).unwrap()).unwrap()
}

#[quickcheck]
fn to_from_string(string: String) -> bool {
    // This creates a new <string> type on a code path that does no XML escaping,
    // so the string needs to be trimmed and XML-escaped first.
    let string = escape(string.trim()).to_string();
    let value = Type::String(string);

    value == from_str::<Type>(&to_string(&value).unwrap()).unwrap()
}

#[quickcheck]
fn from_to_string(string: String) -> bool {
    // This creates a new <string> type on a code path that does no XML escaping,
    // so the string needs to be trimmed and XML-escaped first.
    let string = escape(string.trim()).to_string();
    let value = format!("<string>{string}</string>");

    value == to_string(&from_str::<Type>(&value).unwrap()).unwrap()
}

#[quickcheck]
fn to_from_double(double: f64) -> TestResult {
    if double.is_nan() {
        return TestResult::discard();
    }

    let value = Type::Double(double);

    TestResult::from_bool(value == from_str::<Type>(&to_string(&value).unwrap()).unwrap())
}

#[quickcheck]
fn from_to_double(double: f64) -> TestResult {
    if double.is_nan() {
        return TestResult::discard();
    }

    let value = format!("<double>{double}</double>");

    TestResult::from_bool(value == to_string(&from_str::<Type>(&value).unwrap()).unwrap())
}

#[quickcheck]
fn to_from_base64(bytes: Vec<u8>) -> bool {
    let value = Type::Base64(bytes);

    value == from_str::<Type>(&to_string(&value).unwrap()).unwrap()
}

#[quickcheck]
fn from_to_base64(bytes: Vec<u8>) -> bool {
    let value = format!("<base64>{}</base64>", base64::encode(bytes));

    value == to_string(&from_str::<Type>(&value).unwrap()).unwrap()
}
