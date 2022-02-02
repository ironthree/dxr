#![allow(clippy::unwrap_used)]

use super::{FromDXR, ToDXR};

#[test]
fn roundtrip_struct_empty() {
    #[derive(Debug, PartialEq, FromDXR, ToDXR)]
    struct Test {}

    let value = Test {};
    assert_eq!(Test::from_dxr(&value.to_dxr().unwrap()).unwrap(), value);
}

#[test]
fn roundtrip_struct() {
    #[derive(Debug, PartialEq, FromDXR, ToDXR)]
    struct Test {
        id: i32,
    }

    let value = Test { id: 42 };
    assert_eq!(Test::from_dxr(&value.to_dxr().unwrap()).unwrap(), value);
}

#[test]
fn roundtrip_array() {
    let value = vec![-12, 42];
    assert_eq!(<Vec<i32>>::from_dxr(&value.to_dxr().unwrap()).unwrap(), value);
}

#[test]
fn roundtrip_option_some() {
    let value = Some(42i32);
    assert_eq!(<Option<i32>>::from_dxr(&value.to_dxr().unwrap()).unwrap(), value);
}

#[test]
fn roundtrip_option_none() {
    let value: Option<i32> = None;
    assert_eq!(<Option<i32>>::from_dxr(&value.to_dxr().unwrap()).unwrap(), value);
}
