use quick_xml::{de::from_str, se::to_string};

use crate::values::{Array, Value};

#[test]
fn to_array_empty() {
    let value = Array::new(vec![]);
    let expected = "<array><data/></array>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_array_empty() {
    let value = "<array><data/></array>";
    let expected = Array::new(vec![]);

    assert_eq!(from_str::<Array>(value).unwrap(), expected);

    let value = "<array><data></data></array>";
    let expected = Array::new(vec![]);

    assert_eq!(from_str::<Array>(value).unwrap(), expected);
}

#[test]
fn to_array_one() {
    let value = Array::new(vec![Value::i4(-12)]);
    let expected = "<array><data><value><i4>-12</i4></value></data></array>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_array_one() {
    let value = "<array><data><value><i4>-12</i4></value></data></array>";
    let expected = Array::new(vec![Value::i4(-12)]);

    assert_eq!(from_str::<Array>(value).unwrap(), expected);
}

#[test]
fn to_array_two() {
    let value = Array::new(vec![Value::i4(-12), Value::string("minus twelve")]);
    let expected =
        "<array><data><value><i4>-12</i4></value><value><string>minus twelve</string></value></data></array>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_array_two() {
    let value = "<array><data><value><i4>-12</i4></value><value><string>minus twelve</string></value></data></array>";
    let expected = Array::new(vec![Value::i4(-12), Value::string("minus twelve")]);

    assert_eq!(from_str::<Array>(value).unwrap(), expected);
}

#[test]
fn to_value_array() {
    let value = Value::array(Array::new(vec![Value::i4(-12)]));
    let expected = "<value><array><data><value><i4>-12</i4></value></data></array></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_value_array() {
    let value = "<value><array><data><value><i4>-12</i4></value></data></array></value>";
    let expected = Value::array(Array::new(vec![Value::i4(-12)]));

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}
