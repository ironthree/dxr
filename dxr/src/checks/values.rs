use std::borrow::Cow;

use quick_xml::escape::escape;
use quick_xml::{de::from_str, se::to_string};
use quickcheck::TestResult;
use quickcheck_macros::quickcheck;

use crate::values::Value;
use crate::{TryFromValue, TryToValue};

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
    let value = Value::string(string);

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
    #[allow(deprecated)]
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
    let expected: Cow<str> = Cow::Owned(string.trim().to_owned());
    let value = <Cow<str>>::try_from_value(&TryToValue::try_to_value(&expected).unwrap()).unwrap();

    expected == value
}

#[quickcheck]
fn roundtrip_array(a: i32, b: i32) -> bool {
    let value = vec![a, b];
    <Vec<i32>>::try_from_value(&value.try_to_value().unwrap()).unwrap() == value
}

#[cfg(feature = "derive")]
#[test]
fn roundtrip_struct_empty() {
    use crate::{TryFromValue, TryToValue};

    #[derive(Debug, Eq, PartialEq, TryFromValue, TryToValue)]
    struct Test {}

    let value = Test {};
    assert_eq!(Test::try_from_value(&value.try_to_value().unwrap()).unwrap(), value);
}

#[cfg(all(feature = "derive", feature = "nil"))]
#[quickcheck]
fn roundtrip_struct(int: i32, string: String, boolean: bool, optional: Option<f64>) -> TestResult {
    if matches!(optional, Some(f) if f.is_nan()) {
        return TestResult::discard();
    }

    #[derive(Debug, PartialEq, TryFromValue, TryToValue)]
    struct Test {
        int: i32,
        string: String,
        boolean: bool,
        optional: Option<f64>,
    }

    let value = Test {
        int,
        string: string.trim().to_string(),
        boolean,
        optional,
    };
    TestResult::from_bool(Test::try_from_value(&value.try_to_value().unwrap()).unwrap() == value)
}

#[cfg(feature = "derive")]
#[quickcheck]
fn roundtrip_struct_cow_str(string: String) -> bool {
    #[derive(Debug, Eq, PartialEq, TryFromValue, TryToValue)]
    struct TestCow<'a> {
        string: Cow<'a, str>,
    }

    let expected = TestCow {
        string: Cow::Owned(string.trim().to_owned()),
    };
    let value = TestCow::try_from_value(&TryToValue::try_to_value(&expected).unwrap()).unwrap();

    expected == value
}

#[cfg(feature = "derive")]
#[quickcheck]
fn roundtrip_struct_cow_bytes(bytes: Vec<u8>) -> bool {
    #[derive(Debug, Eq, PartialEq, TryFromValue, TryToValue)]
    struct TestCow<'a> {
        bytes: Cow<'a, Vec<u8>>,
    }

    let expected = TestCow {
        bytes: Cow::Owned(bytes),
    };
    let value = TestCow::try_from_value(&TryToValue::try_to_value(&expected).unwrap()).unwrap();

    expected == value
}

#[cfg(feature = "derive")]
#[quickcheck]
fn roundtrip_struct_cow_static_str(string: String) -> bool {
    #[derive(Debug, Eq, PartialEq, TryFromValue, TryToValue)]
    struct TestCow {
        string: Cow<'static, str>,
    }

    let expected = TestCow {
        string: Cow::Owned(string.trim().to_owned()),
    };
    let value = TestCow::try_from_value(&TryToValue::try_to_value(&expected).unwrap()).unwrap();

    expected == value
}
