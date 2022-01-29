use chrono::{SubsecRound, Utc};

use crate::XML_RPC_DATE_FORMAT;
use crate::{Array, Member, Struct, Type, Value};

//use serde_xml_rs::{from_str, to_string};
use quick_xml::{de::from_str, se::to_string};

#[test]
fn to_i4() {
    let value = Type::Integer(-12);
    let expected = "<i4>-12</i4>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_i4() {
    let value = "<i4>-12</i4>";
    let expected = Type::Integer(-12);

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[test]
fn from_int() {
    let value = "<int>-12</int>";
    let expected = Type::Integer(-12);

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[test]
fn to_value_i4() {
    let value = Value::i4(-12);
    let expected = "<value><i4>-12</i4></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_value_i4() {
    let value = "<value><i4>-12</i4></value>";
    let expected = Value::i4(-12);

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn from_value_int() {
    let value = "<value><int>-12</int></value>";
    let expected = Value::i4(-12);

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn to_i8() {
    let value = Type::Long(-12);
    let expected = "<i8>-12</i8>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn from_i8() {
    let value = "<i8>-12</i8>";
    let expected = Type::Long(-12);

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn to_value_i8() {
    let value = Value::i8(-12);
    let expected = "<value><i8>-12</i8></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[cfg(feature = "i8")]
#[test]
fn from_value_i8() {
    let value = "<value><i8>-12</i8></value>";
    let expected = Value::i8(-12);

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn to_boolean() {
    let value = Type::Boolean(false);
    let expected = "<boolean>0</boolean>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_boolean() {
    let value = "<boolean>0</boolean>";
    let expected = Type::Boolean(false);

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[test]
fn to_value_boolean() {
    let value = Value::boolean(true);
    let expected = "<value><boolean>1</boolean></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_value_boolean() {
    let value = "<value><boolean>1</boolean></value>";
    let expected = Value::boolean(true);

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn to_str() {
    let value = Type::String(String::from("Hello, World!"));
    let expected = "<string>Hello, World!</string>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_string() {
    let value = "<string>Hello, World!</string>";
    let expected = Type::String(String::from("Hello, World!"));

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[test]
fn to_value_string() {
    let value = Value::string(String::from("Hello, World!"));
    let expected = "<value><string>Hello, World!</string></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_value_string() {
    let value = "<value><string>Hello, World!</string></value>";
    let expected = Value::string(String::from("Hello, World!"));

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn to_double() {
    let value = Type::Double(1.5);
    let expected = "<double>1.5</double>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_double() {
    let value = "<double>1.5</double>";
    let expected = Type::Double(1.5);

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[test]
fn to_value_double() {
    let value = Value::double(1.5);
    let expected = "<value><double>1.5</double></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_value_double() {
    let value = "<value><double>1.5</double></value>";
    let expected = Value::double(1.5);

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn to_datetime() {
    let datetime = Utc::now();
    let datetime_str = datetime.format(XML_RPC_DATE_FORMAT).to_string();

    let value = Type::DateTime(datetime);
    let expected = format!("<dateTime.iso8601>{}</dateTime.iso8601>", datetime_str);

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_datetime() {
    let datetime = Utc::now().round_subsecs(0);
    let datetime_str = datetime.format(XML_RPC_DATE_FORMAT).to_string();

    let value = format!("<dateTime.iso8601>{}</dateTime.iso8601>", datetime_str);
    let expected = Type::DateTime(datetime);

    assert_eq!(from_str::<Type>(&value).unwrap(), expected);
}

#[test]
fn to_value_datetime() {
    let datetime = Utc::now();
    let datetime_str = datetime.format(XML_RPC_DATE_FORMAT).to_string();

    let value = Value::datetime(datetime);
    let expected = format!("<value><dateTime.iso8601>{}</dateTime.iso8601></value>", datetime_str);

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_value_datetime() {
    let datetime = Utc::now().round_subsecs(0);
    let datetime_str = datetime.format(XML_RPC_DATE_FORMAT).to_string();

    let value = format!("<value><dateTime.iso8601>{}</dateTime.iso8601></value>", datetime_str);
    let expected = Value::datetime(datetime);

    assert_eq!(from_str::<Value>(&value).unwrap(), expected);
}

#[test]
fn to_base64() {
    let contents = b"you can't read this!";
    let encoded = base64::encode(contents);

    let value = Type::Base64(contents.to_vec());
    let expected = format!("<base64>{}</base64>", encoded);

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_base64() {
    let contents = b"you can't read this!";
    let encoded = base64::encode(contents);

    let value = format!("<base64>{}</base64>", encoded);
    let expected = Type::Base64(contents.to_vec());

    assert_eq!(from_str::<Type>(&value).unwrap(), expected);
}

#[test]
fn to_value_base64() {
    let contents = b"you can't read this!";
    let encoded = base64::encode(contents);

    let value = Value::base64(contents.to_vec());
    let expected = format!("<value><base64>{}</base64></value>", encoded);

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_value_base64() {
    let contents = b"you can't read this!";
    let encoded = base64::encode(contents);

    let value = format!("<value><base64>{}</base64></value>", encoded);
    let expected = Value::base64(contents.to_vec());

    assert_eq!(from_str::<Value>(&value).unwrap(), expected);
}

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
    let value = Struct::from_members(vec![]);
    let expected = "<struct/>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_struct_empty() {
    let value = "<struct/>";
    let expected = Struct::from_members(vec![]);

    assert_eq!(from_str::<Struct>(value).unwrap(), expected);

    let value = "<struct></struct>";
    let expected = Struct::from_members(vec![]);

    assert_eq!(from_str::<Struct>(value).unwrap(), expected);
}

#[test]
fn to_struct_one() {
    let value = Struct::from_members(vec![Member::new(String::from("answer"), Value::i4(42))]);
    let expected = "<struct><member><name>answer</name><value><i4>42</i4></value></member></struct>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_struct_one() {
    let value = "<struct><member><name>answer</name><value><i4>42</i4></value></member></struct>";
    let expected = Struct::from_members(vec![Member::new(String::from("answer"), Value::i4(42))]);

    assert_eq!(from_str::<Struct>(value).unwrap(), expected);
}

#[test]
fn to_struct_two() {
    let value = Struct::from_members(vec![
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
    let expected = Struct::from_members(vec![
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
    let value = Value::structure(Struct::from_members(vec![Member::new(
        String::from("answer"),
        Value::i4(42),
    )]));
    let expected = "<value><struct><member><name>answer</name><value><i4>42</i4></value></member></struct></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_value_struct() {
    let value = "<value><struct><member><name>answer</name><value><i4>42</i4></value></member></struct></value>";
    let expected = Value::structure(Struct::from_members(vec![Member::new(
        String::from("answer"),
        Value::i4(42),
    )]));

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[test]
fn to_array_empty() {
    let value = Array::from_elements(vec![]);
    let expected = "<array><data/></array>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_array_empty() {
    let value = "<array><data/></array>";
    let expected = Array::from_elements(vec![]);

    assert_eq!(from_str::<Array>(value).unwrap(), expected);

    let value = "<array><data></data></array>";
    let expected = Array::from_elements(vec![]);

    assert_eq!(from_str::<Array>(value).unwrap(), expected);
}

#[test]
fn to_array_one() {
    let value = Array::from_elements(vec![Value::i4(-12)]);
    let expected = "<array><data><value><i4>-12</i4></value></data></array>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_array_one() {
    let value = "<array><data><value><i4>-12</i4></value></data></array>";
    let expected = Array::from_elements(vec![Value::i4(-12)]);

    assert_eq!(from_str::<Array>(value).unwrap(), expected);
}

#[test]
fn to_array_two() {
    let value = Array::from_elements(vec![Value::i4(-12), Value::string(String::from("minus twelve"))]);
    let expected =
        "<array><data><value><i4>-12</i4></value><value><string>minus twelve</string></value></data></array>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_array_two() {
    let value = "<array><data><value><i4>-12</i4></value><value><string>minus twelve</string></value></data></array>";
    let expected = Array::from_elements(vec![Value::i4(-12), Value::string(String::from("minus twelve"))]);

    assert_eq!(from_str::<Array>(value).unwrap(), expected);
}

#[test]
fn to_value_array() {
    let value = Value::array(Array::from_elements(vec![Value::i4(-12)]));
    let expected = "<value><array><data><value><i4>-12</i4></value></data></array></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_value_array() {
    let value = "<value><array><data><value><i4>-12</i4></value></data></array></value>";
    let expected = Value::array(Array::from_elements(vec![Value::i4(-12)]));

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn to_nil() {
    let value = Type::Nil;
    let expected = "<nil/>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn from_nil() {
    let value = "<nil/>";
    let expected = Type::Nil;

    assert_eq!(from_str::<Type>(value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn to_value_nil() {
    let value = Value::nil();
    let expected = "<value><nil/></value>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[cfg(feature = "nil")]
#[test]
fn from_value_nil() {
    let value = "<value><nil/></value>";
    let expected = Value::nil();

    assert_eq!(from_str::<Value>(value).unwrap(), expected);
}
