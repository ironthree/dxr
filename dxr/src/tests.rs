#![allow(clippy::unwrap_used)]

#[cfg(feature = "derive")]
#[test]
fn roundtrip_struct_empty() {
    use crate::{TryFromValue, TryToValue};

    #[derive(Debug, Eq, PartialEq, TryFromValue, TryToValue)]
    struct Test {}

    let value = Test {};
    assert_eq!(Test::try_from_value(&value.try_to_value().unwrap()).unwrap(), value);
}
