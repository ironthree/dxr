//! definitions of XML-RPC data types with (de)serialization implementations

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::fault::Fault;

// imports for intra-doc links
#[cfg(doc)]
use crate::{TryFromValue, TryToValue};
#[cfg(doc)]
use std::collections::HashMap;

/// # XML-RPC value type
///
/// The [`Value`] type is the Rust equivalent of valid XML-RPC values. It provides constructors
/// from all compatible primitive types, (de)serialization support from and to XML-RPC value
/// strings, and fallible conversion from and to [`Value`] with implementations of the
/// [`TryFromValue`] and [`TryToValue`] traits.
///
/// In general, using methods from the fallible [`TryFromValue`] and [`TryToValue`] conversion
/// traits is recommended, as they provide a consistent interface across all types, including
/// [`Vec`], arrays, slices, tuples, [`HashMap`]s, and even custom structs, when using the
/// [`TryFromValue`] and [`TryToValue`] derive macros (or implementing the traits manually).
#[derive(Clone, Debug, PartialEq, Serialize)]
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
    /// Support for `<i8>` values is optional and can be enabled with the `i8` crate feature.
    #[cfg(feature = "i8")]
    pub fn i8(value: i64) -> Value {
        Value::new(Type::Long(value))
    }

    /// constructor for `<boolean>` values (true or false)
    pub fn boolean(value: bool) -> Value {
        Value::new(Type::Boolean(value))
    }

    /// constructor for `<string>` values
    pub fn string(value: String) -> Value {
        Value::new(Type::String(value))
    }

    /// constructor for `<double>` values (64-bit floating point numbers)
    pub fn double(value: f64) -> Value {
        Value::new(Type::Double(value))
    }

    /// constructor for `<dateTime.iso8601>` values (timezone-unaware date & time)
    ///
    /// Note that the date & time format used by XML-RPC does not include sub-second precision, nor
    /// any timezone information.
    pub fn datetime(value: NaiveDateTime) -> Value {
        Value::new(Type::DateTime(value))
    }

    /// constructor for `<base64>` values (base64-encoded arbitrary bytes)
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
    /// Support for `<nil>` values is optional and can be enabled with the `nil` crate feature.
    ///
    /// If enabled, this type is used to emulate support for optional values in XML-RPC, by mapping
    /// Rust [`Option`]s to either their contained [`Value`] if the value is [`Some<T>`], or to a
    /// `<nil/>` the value is [`None`].
    ///
    /// This is consistent with the XML-RPC implementation in the Python `xmlrpc` standard library
    /// module, which also maps `None` values to `<nil/>`.
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
    #[serde(rename = "boolean", with = "super::ser_de::boolean")]
    Boolean(#[serde(rename = "$value")] bool),
    #[serde(rename = "string")]
    String(#[serde(rename = "$value")] String),
    #[serde(rename = "double")]
    Double(#[serde(rename = "$value")] f64),
    #[serde(rename = "dateTime.iso8601", with = "super::ser_de::datetime")]
    DateTime(#[serde(rename = "$value")] NaiveDateTime),
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "struct")]
pub(crate) struct Struct {
    #[serde(default, rename = "member")]
    members: Vec<Member>,
}

impl Struct {
    pub(crate) fn new(mut members: Vec<Member>) -> Struct {
        members.sort_by(|a, b| a.name.name.cmp(&b.name.name));
        Struct { members }
    }
}

// custom PartialEq impl: the order of struct members is irrelevant
impl PartialEq for Struct {
    fn eq(&self, other: &Self) -> bool {
        // fast path: different numbers of members
        if self.members.len() != other.members.len() {
            return false;
        }

        // sort members by name before comparing
        let mut self_members: Vec<&Member> = self.members.iter().map(|m| m).collect();
        let mut other_members: Vec<&Member> = other.members.iter().map(|m| m).collect();
        self_members.sort_by(|a, b| a.name.name.cmp(&b.name.name));
        other_members.sort_by(|a, b| a.name.name.cmp(&b.name.name));

        self_members == other_members
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "member")]
pub(crate) struct Member {
    name: MemberName,
    value: Value,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
///
/// The `dxr_client::Call::as_xml_rpc` method from the `dxr_client` crate provides a convenient
/// way of constructing new [`MethodCall`] values that does not require converting method call
/// paramters into [`Value`]s manually.
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
    pub fn new(name: String, params: Vec<Value>) -> MethodCall {
        MethodCall {
            name: MethodName { name },
            params: RequestParameters {
                params: params.into_iter().map(|value| RequestParameter { value }).collect(),
            },
        }
    }

    /// getter method for the method name
    pub fn name(&self) -> &str {
        &self.name.name
    }

    /// extract the list of parameters
    pub fn params(self) -> Vec<Value> {
        self.params.params.into_iter().map(|param| param.value).collect()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename = "methodName")]
struct MethodName {
    #[serde(rename = "$value")]
    name: String,
}

/// # XML-RPC method response type
///
/// The [`MethodResponse`] type is the Rust equivalent of the contents of an XML-RPC response.
///
/// It contains exactly one return value as a parameter.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "methodResponse")]
pub struct MethodResponse {
    params: ResponseParameters,
}

impl MethodResponse {
    /// constructor for `<methodResponse>` values from the return value
    pub fn new(value: Value) -> MethodResponse {
        MethodResponse {
            params: ResponseParameters {
                params: ResponseParameter { params: value },
            },
        }
    }

    /// getter method for the returned value
    pub fn inner(self) -> Value {
        self.params.params.params
    }
}

/// # XML-RPC fault response type
///
/// The [`FaultResponse`] type is the Rust equivalent of the contents of an XML-RPC fault response.
/// Values of this type can be constructed from [`Fault`]s:
///
/// ```
/// use dxr::{Fault, FaultResponse};
///
/// let fault = Fault::new(404, String::from("Not Found"));
/// let _response: FaultResponse = fault.into();
/// ```
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
    params: Vec<RequestParameter>,
}

impl RequestParameters {
    fn is_empty(&self) -> bool {
        self.params.is_empty()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "param")]
struct RequestParameter {
    value: Value,
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
    #[serde(rename = "value")]
    params: Value,
}
