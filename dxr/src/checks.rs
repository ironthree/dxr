#![allow(unused_imports)]
#![allow(clippy::unwrap_used)]

use std::borrow::Cow;

use quickcheck::TestResult;
use quickcheck_macros::quickcheck;

use crate::{TryFromValue, TryToValue};

#[cfg(feature = "derive")]
#[quickcheck]
fn roundtrip_struct_cow_static(string: String) -> bool {
    #[derive(Debug, Eq, PartialEq, TryFromValue, TryToValue)]
    struct TestCow {
        string: Cow<'static, String>,
    }

    let expected = TestCow {
        string: Cow::Owned(string.trim().to_owned()),
    };
    let value = TestCow::try_from_value(&TryToValue::try_to_value(&expected).unwrap()).unwrap();

    expected == value
}

#[cfg(feature = "derive")]
#[quickcheck]
fn roundtrip_struct_cow_string(string: String) -> bool {
    #[derive(Debug, Eq, PartialEq, TryFromValue, TryToValue)]
    struct TestCow<'a> {
        string: Cow<'a, String>,
    }

    let expected = TestCow {
        string: Cow::Owned(string.trim().to_owned()),
    };
    let value = TestCow::try_from_value(&TryToValue::try_to_value(&expected).unwrap()).unwrap();

    expected == value
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
