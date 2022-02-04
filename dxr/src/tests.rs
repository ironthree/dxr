#![allow(clippy::unwrap_used)]

use quickcheck::TestResult;
use quickcheck_macros::quickcheck;

use crate::{FromDXR, ToDXR};

#[cfg(feature = "derive")]
#[test]
fn roundtrip_struct_empty() {
    #[derive(Debug, PartialEq, FromDXR, ToDXR)]
    struct Test {}

    let value = Test {};
    assert_eq!(Test::from_dxr(&value.to_dxr().unwrap()).unwrap(), value);
}

#[cfg(feature = "derive")]
#[quickcheck]
fn roundtrip_struct(int: i32, string: String, boolean: bool, optional: Option<f64>) -> TestResult {
    if matches!(optional, Some(f) if f.is_nan()) {
        return TestResult::discard();
    }

    #[derive(Debug, PartialEq, FromDXR, ToDXR)]
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
    TestResult::from_bool(Test::from_dxr(&value.to_dxr().unwrap()).unwrap() == value)
}

#[quickcheck]
fn roundtrip_array(a: i32, b: i32) -> bool {
    let value = vec![a, b];
    <Vec<i32>>::from_dxr(&value.to_dxr().unwrap()).unwrap() == value
}

#[cfg(feature = "nil")]
#[test]
fn roundtrip_option_none() {
    let value: Option<i32> = None;
    assert_eq!(<Option<i32>>::from_dxr(&value.to_dxr().unwrap()).unwrap(), value);
}

#[cfg(feature = "nil")]
#[quickcheck]
fn roundtrip_option_some(a: i32) -> bool {
    let value = Some(a);
    <Option<i32>>::from_dxr(&value.to_dxr().unwrap()).unwrap() == value
}
