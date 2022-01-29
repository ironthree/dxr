//#![warn(missing_docs)]
//#![warn(missing_debug_implementations)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod ser_de;

pub const XML_RPC_DATE_FORMAT: &str = "%Y%m%dT%H:%M:%S";

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename = "value")]
pub struct Value {
    #[serde(rename = "$value")]
    pub value: Type,
}

impl Value {
    pub fn i4(value: i32) -> Value {
        Value {
            value: Type::Integer(value),
        }
    }

    pub fn int(value: i32) -> Value {
        Value::i4(value)
    }

    #[cfg(feature = "i8")]
    pub fn i8(value: i64) -> Value {
        Value {
            value: Type::Long(value),
        }
    }

    pub fn boolean(value: bool) -> Value {
        Value {
            value: Type::Boolean(value),
        }
    }

    pub fn string(value: String) -> Value {
        Value {
            value: Type::String(value),
        }
    }

    pub fn double(value: f64) -> Value {
        Value {
            value: Type::Double(value),
        }
    }

    pub fn datetime(value: DateTime<Utc>) -> Value {
        Value {
            value: Type::DateTime(value),
        }
    }

    pub fn base64(value: Vec<u8>) -> Value {
        Value {
            value: Type::Base64(value),
        }
    }

    #[cfg(feature = "nil")]
    pub fn nil() -> Value {
        Value {
            value: Type::Nil,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
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
    Struct(#[serde(rename = "$value")] Struct),
    #[serde(rename = "array")]
    Array(#[serde(rename = "$value")] Array),
    #[cfg(feature = "nil")]
    #[serde(rename = "nil")]
    Nil,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Struct {
    #[serde(rename = "$value")]
    pub members: Vec<Member>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Member {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Array {
    #[serde(rename = "$value")]
    pub data: Vec<Element>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Element {
    #[serde(rename = "$value")]
    pub value: Value,
}

#[cfg(test)]
mod tests;
