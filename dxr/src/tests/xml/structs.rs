use crate::values::{Member, Struct, Value};
use crate::xml::{deserialize_xml as from_str, serialize_xml as to_string};

#[test]
fn to_member() {
    let value = Member::new(String::from("answer"), Value::i4(42));
    let expected = "<member><name>answer</name><value><i4>42</i4></value></member>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_member() {
    let value = "<member><name>answer</name><value><i4>42</i4></value></member>";
    let expected = Member::new(String::from("answer"), Value::i4(42));

    assert_eq!(from_str::<Member>(value).unwrap(), expected);
}

#[test]
fn to_struct_empty() {
    let value = Struct::new(vec![]);
    let expected = "<struct/>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_struct_empty() {
    let value = "<struct/>";
    let expected = Struct::new(vec![]);

    assert_eq!(from_str::<Struct>(value).unwrap(), expected);

    let value = "<struct></struct>";
    let expected = Struct::new(vec![]);

    assert_eq!(from_str::<Struct>(value).unwrap(), expected);
}

#[test]
fn to_struct_one() {
    let value = Struct::new(vec![Member::new(String::from("answer"), Value::i4(42))]);
    let expected = "<struct><member><name>answer</name><value><i4>42</i4></value></member></struct>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_struct_one() {
    let value = "<struct><member><name>answer</name><value><i4>42</i4></value></member></struct>";
    let expected = Struct::new(vec![Member::new(String::from("answer"), Value::i4(42))]);

    assert_eq!(from_str::<Struct>(value).unwrap(), expected);
}

#[test]
fn to_struct_two() {
    let value = Struct::new(vec![
        Member::new(String::from("answer"), Value::i4(42)),
        Member::new(
            String::from("question"),
            Value::string(String::from("The answer to life, the the universe, and everything")),
        ),
    ]);
    let expected = "<struct><member><name>answer</name><value><i4>42</i4></value></member><member><name>question</name><value><string>The answer to life, the the universe, and everything</string></value></member></struct>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_struct_two() {
    let value = "<struct><member><name>answer</name><value><i4>42</i4></value></member><member><name>question</name><value><string>The answer to life, the the universe, and everything</string></value></member></struct>";
    let expected = Struct::new(vec![
        Member::new(String::from("answer"), Value::i4(42)),
        Member::new(
            String::from("question"),
            Value::string(String::from("The answer to life, the the universe, and everything")),
        ),
    ]);

    assert_eq!(from_str::<Struct>(value).unwrap(), expected);
}

#[test]
fn to_value_struct() {
    let value = Value::structure(Struct::new(vec![Member::new(String::from("answer"), Value::i4(42))]));
    let expected = "<value><struct><member><name>answer</name><value><i4>42</i4></value></member></struct></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_value_struct() {
    let value = "<value><struct><member><name>answer</name><value><i4>42</i4></value></member></struct></value>";
    let expected = Value::structure(Struct::new(vec![Member::new(String::from("answer"), Value::i4(42))]));

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn roundtrip_option_none() {
    use crate::{TryFromValue, TryToValue};

    let value: Option<i32> = None;
    assert_eq!(
        <Option<i32>>::try_from_value(&value.try_to_value().unwrap()).unwrap(),
        value
    );
}
