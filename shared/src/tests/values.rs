use super::*;

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
