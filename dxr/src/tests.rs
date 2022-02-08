#![allow(clippy::unwrap_used)]

#[cfg(all(feature = "derive", feature = "nil"))]
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

#[cfg(all(feature = "derive", feature = "nil"))]
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

#[cfg(feature = "server")]
#[test]
fn server_builder_debug() {
    use crate::ServerBuilder;

    let builder = ServerBuilder::new("0.0.0.0:3000".parse().unwrap());
    insta::assert_debug_snapshot!(builder);
}

#[cfg(all(feature = "server", feature = "nil"))]
#[test]
fn server_builder_debug_with_method() {
    use crate::{HandlerFn, HandlerResult, ServerBuilder, Value};
    use axum::http::HeaderMap;

    fn noop(_v: &[Value], _h: &HeaderMap) -> HandlerResult {
        Ok(None)
    }

    let builder = ServerBuilder::new("0.0.0.0:3000".parse().unwrap()).add_method("noop", Box::new(noop as HandlerFn));
    insta::assert_debug_snapshot!(builder);
}

#[cfg(all(feature = "server", feature = "tokio"))]
#[test]
fn server_builder_debug_with_off_switch() {
    use crate::{ServerBuilder, TokioOffSwitch};

    let off_switch = TokioOffSwitch::new();
    let builder = ServerBuilder::new("0.0.0.0:3000".parse().unwrap()).add_off_switch(Box::new(off_switch));
    insta::assert_debug_snapshot!(builder);
}

#[cfg(feature = "server")]
#[test]
fn server_debug() {
    use crate::ServerBuilder;

    let server = ServerBuilder::new("0.0.0.0:3000".parse().unwrap()).build();
    insta::assert_debug_snapshot!(server);
}

#[cfg(all(feature = "server", feature = "nil"))]
#[test]
fn server_debug_with_method() {
    use crate::{HandlerFn, HandlerResult, ServerBuilder, Value};
    use axum::http::HeaderMap;

    fn noop(_v: &[Value], _h: &HeaderMap) -> HandlerResult {
        Ok(None)
    }

    let server = ServerBuilder::new("0.0.0.0:3000".parse().unwrap())
        .add_method("noop", Box::new(noop as HandlerFn))
        .build();
    insta::assert_debug_snapshot!(server);
}

#[cfg(all(feature = "server", feature = "tokio"))]
#[test]
fn server_debug_with_off_switch() {
    use crate::{ServerBuilder, TokioOffSwitch};

    let off_switch = TokioOffSwitch::new();
    let server = ServerBuilder::new("0.0.0.0:3000".parse().unwrap())
        .add_off_switch(Box::new(off_switch))
        .build();
    insta::assert_debug_snapshot!(server);
}
