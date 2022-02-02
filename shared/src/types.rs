//! definitions of XML-RPC data types with (de)serialization implementations

use chrono::{DateTime, Utc};
use quick_xml::escape::{escape, unescape};
use serde::{Deserialize, Serialize};

use crate::error::DxrError;
use crate::fault::Fault;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "value")]
pub struct Value {
    #[serde(rename = "$value")]
    value: Type,
}

impl Value {
    fn new(value: Type) -> Value {
        Value { value }
    }

    pub(crate) fn inner(&self) -> &Type {
        &self.value
    }

    pub fn i4(value: i32) -> Value {
        Value::new(Type::Integer(value))
    }

    #[cfg(feature = "i8")]
    pub fn i8(value: i64) -> Value {
        Value::new(Type::Long(value))
    }

    pub fn boolean(value: bool) -> Value {
        Value::new(Type::Boolean(value))
    }

    pub(crate) fn string(value: String) -> Value {
        Value::new(Type::String(value))
    }

    /// constructor for a [`Value`] of type string that handles escaping input for XML
    pub fn string_escape(value: &str) -> Result<Value, DxrError> {
        let string = String::from_utf8(escape(value.trim().as_bytes()).to_vec())
            .map_err(|error| DxrError::invalid_data(error.to_string()))?;
        Ok(Value::string(string))
    }

    /// associated method un-escaping a [`Value`] of type string
    pub fn string_unescape(value: &str) -> Result<String, DxrError> {
        match unescape(value.as_bytes()) {
            Ok(bytes) => String::from_utf8(bytes.to_vec()).map_err(|error| DxrError::InvalidData {
                error: error.to_string(),
            }),
            Err(error) => Err(DxrError::InvalidData {
                error: error.to_string(),
            }),
        }
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

    pub(crate) fn structure(value: Struct) -> Value {
        Value::new(Type::Struct { members: value.members })
    }

    pub(crate) fn array(value: Array) -> Value {
        Value::new(Type::Array { data: value.data })
    }

    #[cfg(feature = "nil")]
    pub fn nil() -> Value {
        Value::new(Type::Nil)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) enum Type {
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

impl Type {
    pub(crate) fn name(&self) -> &'static str {
        match self {
            Type::Integer(_) => "i4",
            #[cfg(feature = "i8")]
            Type::Long(_) => "i8",
            Type::Boolean(_) => "boolean",
            Type::String(_) => "string",
            Type::Double(_) => "double",
            Type::DateTime(_) => "dateTime.iso8601",
            Type::Base64(_) => "base64",
            Type::Struct { .. } => "struct",
            Type::Array { .. } => "array",
            #[cfg(feature = "nil")]
            Type::Nil => "nil",
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "struct")]
pub(crate) struct Struct {
    #[serde(default, rename = "member")]
    members: Vec<Member>,
}

impl Struct {
    pub(crate) fn new(members: Vec<Member>) -> Struct {
        Struct { members }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "member")]
pub(crate) struct Member {
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
    pub(crate) fn new(name: String, value: Value) -> Member {
        Member {
            name: MemberName { name },
            value,
        }
    }

    pub(crate) fn name(&self) -> &str {
        self.name.name.as_str()
    }

    pub(crate) fn inner(&self) -> &Value {
        &self.value
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "array")]
pub(crate) struct Array {
    #[serde(default)]
    data: ArrayData,
}

impl Array {
    pub(crate) fn new(values: Vec<Value>) -> Array {
        Array {
            data: ArrayData { values },
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename = "data")]
pub(crate) struct ArrayData {
    #[serde(default, rename = "value")]
    values: Vec<Value>,
}

impl ArrayData {
    pub(crate) fn inner(&self) -> &Vec<Value> {
        &self.values
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "methodCall")]
pub struct MethodCall {
    #[serde(rename = "methodName")]
    name: MethodName,
    #[serde(default, skip_serializing_if = "RequestParameters::is_empty")]
    params: RequestParameters,
}

impl MethodCall {
    pub fn new(name: String, parameters: Vec<Value>) -> MethodCall {
        MethodCall {
            name: MethodName { name },
            params: RequestParameters {
                params: ParameterData { params: parameters },
            },
        }
    }

    pub fn name(&self) -> &str {
        &self.name.name
    }

    pub fn params(&self) -> &Vec<Value> {
        &self.params.params.params
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
    params: ResponseParameters,
}

impl MethodResponse {
    pub fn new(value: Value) -> MethodResponse {
        MethodResponse {
            params: ResponseParameters {
                params: ResponseParameter { value },
            },
        }
    }

    pub fn inner(self) -> Value {
        self.params.params.value
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "methodResponse")]
pub struct FaultResponse {
    fault: FaultStruct,
}

impl FaultResponse {
    pub(crate) fn members(&self) -> &[Member] {
        &self.fault.value.value.members
    }
}

impl From<Fault> for FaultResponse {
    fn from(fault: Fault) -> Self {
        FaultResponse {
            fault: FaultStruct {
                value: FaultValue {
                    value: Struct::new(vec![
                        Member::new(String::from("faultCode"), Value::i4(fault.code())),
                        Member::new(String::from("faultString"), Value::string(fault.string().to_owned())),
                    ]),
                },
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "fault")]
struct FaultStruct {
    value: FaultValue,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "value")]
struct FaultValue {
    #[serde(rename = "struct")]
    value: Struct,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename = "params")]
struct RequestParameters {
    #[serde(default, rename = "param")]
    params: ParameterData,
}

impl RequestParameters {
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "params")]
struct ResponseParameters {
    #[serde(rename = "param")]
    params: ResponseParameter,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "param")]
struct ResponseParameter {
    value: Value,
}
