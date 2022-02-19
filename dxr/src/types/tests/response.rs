use quick_xml::{de::from_str, se::to_string};

use crate::types::fault::Fault;
use crate::types::structs::{FaultResponse, MethodResponse, Value};

#[test]
fn to_method_response_empty() {
    let value = MethodResponse::empty();
    let expected = "<methodResponse/>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_method_response_empty() {
    let value = "<methodResponse/>";
    let expected = MethodResponse::empty();

    assert_eq!(from_str::<MethodResponse>(value).unwrap(), expected);

    let value = "<methodResponse></methodResponse>";
    let expected = MethodResponse::empty();

    assert_eq!(from_str::<MethodResponse>(value).unwrap(), expected);
}

#[test]
fn to_method_response_success() {
    let value = MethodResponse::new(Value::string(String::from("Success!")));
    let expected =
        "<methodResponse><params><param><value><string>Success!</string></value></param></params></methodResponse>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_method_response_success() {
    let value =
        "<methodResponse><params><param><value><string>Success!</string></value></param></params></methodResponse>";
    let expected = MethodResponse::new(Value::string(String::from("Success!")));

    assert_eq!(from_str::<MethodResponse>(value).unwrap(), expected);
}

#[test]
fn to_method_response_fault() {
    let value = FaultResponse::from(Fault::new(4, String::from("Too many parameters.")));
    let expected = "<methodResponse><fault><value><struct><member><name>faultCode</name><value><i4>4</i4></value></member><member><name>faultString</name><value><string>Too many parameters.</string></value></member></struct></value></fault></methodResponse>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_method_response_fault() {
    let value = "<methodResponse><fault><value><struct><member><name>faultCode</name><value><i4>4</i4></value></member><member><name>faultString</name><value><string>Too many parameters.</string></value></member></struct></value></fault></methodResponse>";
    let expected = FaultResponse::from(Fault::new(4, String::from("Too many parameters.")));

    assert_eq!(from_str::<FaultResponse>(value).unwrap(), expected);
}
