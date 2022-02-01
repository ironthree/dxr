use super::*;

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
fn to_method_call() {
    let value = MethodCall::new(String::from("hello"), vec![Value::string(String::from("xmlrpc"))]);
    let expected = "<methodCall><methodName>hello</methodName><params><param><value><string>xmlrpc</string></value></param></params></methodCall>";

    assert_eq!(to_string(&value).unwrap(), expected);
}

#[test]
fn from_method_call() {
    let value = "<methodCall><methodName>hello</methodName><params><param><value><string>xmlrpc</string></value></param></params></methodCall>";
    let expected = MethodCall::new(String::from("hello"), vec![Value::string(String::from("xmlrpc"))]);

    assert_eq!(from_str::<MethodCall>(value).unwrap(), expected);
}
