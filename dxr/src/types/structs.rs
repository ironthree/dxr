//! definitions of XML-RPC data types with (de)serialization implementations

use chrono::{DateTime, Utc};
use quick_xml::escape::{escape, unescape};
use serde::{Deserialize, Serialize};

use crate::types::error::DxrError;
use crate::types::fault::Fault;

/// # XML-RPC value type
///
/// The [`Value`] type is the Rust equivalent of valid XML-RPC values. It provides constructors
/// from all compatible primitive types, (de)serialization support from and to XML-RPC value
/// strings, and fallible conversion from and to [`Value`] with implementations of the
/// `FromDXR` and `ToDXR` traits.
///
/// Note that the constructors for all primitive value types are infallible, except for the string
/// type, which can fail if the string argument fails to be escaped properly for XML.
///
/// In general, using methods from the fallible `FromDXR` and `ToDXR` conversion traits is
/// recommended, as they provide a consistent interface across all types, including [`Vec`],
/// arrays, slices, tuples, `HashMap`s, and even custom structs, when using the `FromDXR` and
/// `ToDXR` derive macros.
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

    /// constructor for `<i4>` values (signed 32-bit integers)
    pub fn i4(value: i32) -> Value {
        Value::new(Type::Integer(value))
    }

    /// constructor for `<i8>` values (signed 64-bit integers)
    ///
    /// This type is not part of the original XML-RPC spec, but is a widely used extension.
    /// Support for `<i8>` values is optional, but enabled by default.
    #[cfg(feature = "i8")]
    #[cfg_attr(docsrs, doc(cfg(feature = "i8")))]
    pub fn i8(value: i64) -> Value {
        Value::new(Type::Long(value))
    }

    /// constructor for `<boolean>` values (true or false)
    pub fn boolean(value: bool) -> Value {
        Value::new(Type::Boolean(value))
    }

    pub(crate) fn string(value: String) -> Value {
        Value::new(Type::String(value))
    }

    /// constructor for `<string>` values
    ///
    /// Note that this constructor handles string escaping for safe inclusion in XML internally.
    /// Using the `FromDXR` and `ToDXR` trait implementations for [`String`] and [`&str`][str]
    /// is recommended, as those handle escaping and un-escaping automatically.
    pub fn string_escape(value: &str) -> Result<Value, DxrError> {
        let string = String::from_utf8(escape(value.trim().as_bytes()).to_vec())
            .map_err(|error| DxrError::invalid_data(error.to_string()))?;
        Ok(Value::string(string))
    }

    pub(crate) fn string_unescape(value: &str) -> Result<String, DxrError> {
        match unescape(value.as_bytes()) {
            Ok(bytes) => String::from_utf8(bytes.to_vec()).map_err(|error| DxrError::invalid_data(error.to_string())),
            Err(error) => Err(DxrError::invalid_data(error.to_string())),
        }
    }

    /// constructor for `<double>` values (64-bit floating point numbers)
    pub fn double(value: f64) -> Value {
        Value::new(Type::Double(value))
    }

    /// constructor for `<dateTime.iso8601>` values (date & time)
    ///
    /// Note that the date & time format used by XML-RPC does not include sub-second precision, nor
    /// any timezone information. This crate assumes [`Utc`] is used on the server.
    pub fn datetime(value: DateTime<Utc>) -> Value {
        Value::new(Type::DateTime(value))
    }

    /// constructor for `<base64>` values (base64-encoded, arbitrary bytes)
    pub fn base64(value: Vec<u8>) -> Value {
        Value::new(Type::Base64(value))
    }

    pub(crate) fn structure(value: Struct) -> Value {
        Value::new(Type::Struct { members: value.members })
    }

    pub(crate) fn array(value: Array) -> Value {
        Value::new(Type::Array { data: value.data })
    }

    /// constructor for the `<nil/>` value (empty / missing value)
    ///
    /// This type is not part of the original XML-RPC spec, but is a widely used extension.
    /// Support for `<nil>` values is optional, but enabled by default.
    ///
    /// If enabled, this type is used to emulate support for optional values in XML-RPC, by mapping
    /// Rust [`Option`]s to either their contained [`Value`], or to a `<nil>` value. This is
    /// consistent with the XML-RPC implementation in the Python `xmlrpc` standard library module.
    #[cfg(feature = "nil")]
    #[cfg_attr(docsrs, doc(cfg(feature = "nil")))]
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
    #[serde(rename = "boolean", with = "super::ser_de::boolean")]
    Boolean(#[serde(rename = "$value")] bool),
    #[serde(rename = "string")]
    String(#[serde(rename = "$value")] String),
    #[serde(rename = "double")]
    Double(#[serde(rename = "$value")] f64),
    #[serde(rename = "dateTime.iso8601", with = "super::ser_de::datetime")]
    DateTime(#[serde(rename = "$value")] DateTime<Utc>),
    #[serde(rename = "base64", with = "super::ser_de::base64")]
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

/// # XML-RPC method call type
///
/// The [`MethodCall`] type is the Rust equivalent of the contents of an XML-RPC method call.
///
/// It contains the name of the method, and a list of dynamically typed method call parameters.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "methodCall")]
pub struct MethodCall {
    #[serde(rename = "methodName")]
    name: MethodName,
    #[serde(default, skip_serializing_if = "RequestParameters::is_empty")]
    params: RequestParameters,
}

impl MethodCall {
    /// constructor for `<methodCall>` values from method name and parameter list
    pub fn new(name: String, parameters: Vec<Value>) -> MethodCall {
        MethodCall {
            name: MethodName { name },
            params: RequestParameters {
                params: ParameterData { params: parameters },
            },
        }
    }

    /// getter method for the method name
    pub fn name(&self) -> &str {
        &self.name.name
    }

    /// getter method for the list of parameters
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

/// # XML-RPC method response type
///
/// The [`MethodResponse`] type is the Rust equivalent of the contents of an XML-RPC response.
///
/// It contains zero or one return values.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "methodResponse")]
pub struct MethodResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    params: Option<ResponseParameters>,
}

impl MethodResponse {
    /// constructor for `<methodResponse>` values from the return value
    pub fn new(value: Value) -> MethodResponse {
        MethodResponse {
            params: Some(ResponseParameters {
                params: ResponseParameter { value },
            }),
        }
    }

    /// constructor empty `<methodResponse>` values without a value
    pub fn empty() -> MethodResponse {
        MethodResponse { params: None }
    }

    /// getter method for the returned value
    pub fn inner(self) -> Option<Value> {
        self.params.map(|o| o.params.value)
    }
}

/// # XML-RPC fault response type
///
/// The [`FaultResponse`] type is the Rust equivalent of the contents of an XML-RPC fault response.
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
