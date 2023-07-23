use crate::values::{MethodCall, Value};
use crate::xml::{deserialize_xml as from_str, serialize_xml as to_string};

#[test]
fn to_method_call_no_args() {
    let value = MethodCall::new(String::from("hello"), vec![]);
    let expected = "<methodCall><methodName>hello</methodName></methodCall>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_method_call_no_args() {
    let value = "<methodCall><methodName>hello</methodName></methodCall>";
    let expected = MethodCall::new(String::from("hello"), vec![]);

    assert_eq!(from_str::<MethodCall>(value).unwrap(), expected);

    let value = "<methodCall><methodName>hello</methodName><params/></methodCall>";
    let expected = MethodCall::new(String::from("hello"), vec![]);

    assert_eq!(from_str::<MethodCall>(value).unwrap(), expected);
}

#[test]
fn to_method_call_one_arg() {
    let value = MethodCall::new(String::from("hello"), vec![Value::string(String::from("xmlrpc"))]);
    let expected = "<methodCall><methodName>hello</methodName><params><param><value><string>xmlrpc</string></value></param></params></methodCall>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_method_call_one_arg() {
    let value = "<methodCall><methodName>hello</methodName><params><param><value><string>xmlrpc</string></value></param></params></methodCall>";
    let expected = MethodCall::new(String::from("hello"), vec![Value::string(String::from("xmlrpc"))]);

    assert_eq!(from_str::<MethodCall>(value).unwrap(), expected);
}

#[test]
fn to_method_call_two_args() {
    let value = MethodCall::new(String::from("add"), vec![Value::i4(1), Value::i4(1)]);
    let expected = "<methodCall><methodName>add</methodName><params><param><value><i4>1</i4></value></param><param><value><i4>1</i4></value></param></params></methodCall>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_method_call_two_args() {
    let value = "<methodCall><methodName>add</methodName><params><param><value><int>1</int></value></param><param><value><int>1</int></value></param></params></methodCall>";
    let expected = MethodCall::new(String::from("add"), vec![Value::i4(1), Value::i4(1)]);

    assert_eq!(from_str::<MethodCall>(value).unwrap(), expected);
}
