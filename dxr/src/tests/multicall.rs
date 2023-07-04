use quick_xml::{de::from_str, se::to_string};

use crate::values::{MethodCall, Value};

#[test]
fn to_multicall() {
    let value = MethodCall::new(String::from("hello"), vec![]);
    let expected = "<methodCall><methodName>system.multicall</methodName></methodCall>";

    assert_eq!(to_string(&value).unwrap(), expected);
}
