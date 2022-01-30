//! definitions of XML-RPC data types with (de)serialization implementations

#![allow(missing_docs)]

use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "value")]
pub struct Value {
    #[serde(rename = "$value")]
    value: Type,
}

impl Value {
    pub fn new(value: Type) -> Value {
        Value { value }
    }

    pub fn inner(&self) -> &Type {
        &self.value
    }

    pub fn i4(value: i32) -> Value {
        Value::new(Type::Integer(value))
    }

    pub fn int(value: i32) -> Value {
        Value::i4(value)
    }

    #[cfg(feature = "i8")]
    pub fn i8(value: i64) -> Value {
        Value::new(Type::Long(value))
    }

    pub fn boolean(value: bool) -> Value {
        Value::new(Type::Boolean(value))
    }

    pub fn string(value: String) -> Value {
        Value::new(Type::String(value))
    }

    pub fn double(value: f64) -> Value {
        Value::new(Type::Double(value))
    }

    pub fn datetime(value: DateTime<Utc>) -> Value {
        Value::new(Type::DateTime(value))
    }

    pub fn base64(value: Vec<u8>) -> Value {
        Value::new(Type::Base64(value))
    }

    pub fn structure(value: Struct) -> Value {
        Value::new(Type::Struct { members: value.members })
    }

    pub fn array(value: Array) -> Value {
        Value::new(Type::Array { data: value.data })
    }

    #[cfg(feature = "nil")]
    pub fn nil() -> Value {
        Value::new(Type::Nil)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Type {
    #[serde(rename = "i4", alias = "int")]
    Integer(#[serde(rename = "$value")] i32),
    #[cfg(feature = "i8")]
    #[serde(rename = "i8")]
    Long(#[serde(rename = "$value")] i64),
    #[serde(rename = "boolean", with = "crate::ser_de::boolean")]
    Boolean(#[serde(rename = "$value")] bool),
    #[serde(rename = "string")]
    String(#[serde(rename = "$value")] String),
    #[serde(rename = "double")]
    Double(#[serde(rename = "$value")] f64),
    #[serde(rename = "dateTime.iso8601", with = "crate::ser_de::datetime")]
    DateTime(#[serde(rename = "$value")] DateTime<Utc>),
    #[serde(rename = "base64", with = "crate::ser_de::base64")]
    Base64(#[serde(rename = "$value")] Vec<u8>),
    #[serde(rename = "struct")]
    Struct {
        #[serde(default, rename = "member")]
        members: Vec<Member>,
    },
    #[serde(rename = "array")]
    Array {
        #[serde(default)]
        data: ArrayData,
    },
    #[cfg(feature = "nil")]
    #[serde(rename = "nil")]
    Nil,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "struct")]
pub struct Struct {
    #[serde(default, rename = "member")]
    members: Vec<Member>,
}

impl Struct {
    pub fn new(members: Vec<Member>) -> Struct {
        Struct { members }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "member")]
pub struct Member {
    name: MemberName,
    value: Value,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "name")]
struct MemberName {
    #[serde(rename = "$value")]
    name: String,
}

impl Member {
    pub fn new(name: String, value: Value) -> Member {
        Member {
            name: MemberName { name },
            value,
        }
    }

    pub fn name(&self) -> &str {
        self.name.name.as_str()
    }

    pub fn inner(&self) -> &Value {
        &self.value
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "array")]
pub struct Array {
    #[serde(default)]
    data: ArrayData,
}

impl Array {
    pub fn new(values: Vec<Value>) -> Array {
        Array {
            data: ArrayData { values },
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename = "data")]
pub struct ArrayData {
    #[serde(default, rename = "value")]
    values: Vec<Value>,
}

impl ArrayData {
    pub fn inner(&self) -> &Vec<Value> {
        &self.values
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "methodCall")]
pub struct MethodCall {
    #[serde(rename = "methodName")]
    name: MethodName,
    #[serde(default, skip_serializing_if = "Parameters::is_empty")]
    params: Parameters,
}

impl MethodCall {
    pub fn new(name: String, parameters: Vec<Value>) -> MethodCall {
        MethodCall {
            name: MethodName { name },
            params: Parameters {
                params: ParameterData { params: parameters },
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "methodName")]
struct MethodName {
    #[serde(rename = "$value")]
    name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "methodResponse")]
pub struct MethodResponse {
    params: Parameters,
}

impl MethodResponse {
    pub fn new(parameters: Vec<Value>) -> MethodResponse {
        MethodResponse {
            params: Parameters {
                params: ParameterData { params: parameters },
            },
        }
    }

    pub fn into_values(self) -> Vec<Value> {
        self.params.params.params
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "methodResponse")]
pub struct FaultResponse {
    fault: FaultStruct,
}

impl FaultResponse {
    pub fn new(value: Fault) -> FaultResponse {
        FaultResponse {
            fault: FaultStruct {
                value: FaultValue {
                    value: Struct::new(vec![
                        Member::new(String::from("faultCode"), Value::i4(value.code)),
                        Member::new(String::from("faultString"), Value::string(value.string)),
                    ]),
                },
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "fault")]
pub struct FaultStruct {
    value: FaultValue,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "value")]
pub struct FaultValue {
    #[serde(rename = "struct")]
    value: Struct,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fault {
    code: i32,
    string: String,
}

impl Fault {
    pub fn new(code: i32, string: String) -> Fault {
        Fault { code, string }
    }
}

impl Display for Fault {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Fault {}: {}", self.code, self.string)
    }
}

impl From<FaultResponse> for Fault {
    fn from(f: FaultResponse) -> Self {
        let mut members = f.fault.value.value.members;

        let code = if let Type::Integer(code) = members.remove(0).value.value {
            code
        } else {
            unreachable!()
        };

        let string = if let Type::String(string) = members.remove(0).value.value {
            string
        } else {
            unreachable!()
        };

        Fault { code, string }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename = "params")]
pub struct Parameters {
    #[serde(default, rename = "param")]
    params: ParameterData,
}

impl Parameters {
    fn is_empty(&self) -> bool {
        self.params.params.is_empty()
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename = "param")]
struct ParameterData {
    #[serde(rename = "value")]
    params: Vec<Value>,
}
