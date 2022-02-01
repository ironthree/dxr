//! # dxr_shared
//!
//! This crate provides base implementations of all XML-RPC types and functionality that is used in
//! the macros provided by `dxr_derive` and the high-level functionality provided in `dxr` itself.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

// re-exports and modules

/// re-export of chrono, since it is part of the public API
pub use chrono;

mod ser_de;

mod from;
pub use from::*;

mod to;
pub use to::*;

pub mod types;

// definitions and implementations

use crate::types::{Fault, Value};
use std::borrow::Cow;
use thiserror::Error;

/// date & time format used by the XML-RPC `dateTime.iso8601` value type
pub const XML_RPC_DATE_FORMAT: &str = "%Y%m%dT%H:%M:%S";

/// conversion trait from XML-RPC values to primitives, `Option`, `HashMap`, and user-defined types
pub trait FromDXR: Sized {
    /// fallible conversion method from an XML-RPC value into the target type
    ///
    /// If the value contains a type that is not compatible with the target type, the conversion
    /// will fail.
    fn from_dxr(value: &Value) -> Result<Self, DxrError>;
}

/// conversion trait from primitives, `Option`, `HashMap`, and user-defined types to XML-RPC values
pub trait ToDXR {
    /// conversion method from types into XML-RPC values
    ///
    /// The resulting XML-RPC value will automatically have a compatible type, so this conversion
    /// can only fail if strings cannot un-escaped from XML correctly.
    fn to_dxr(&self) -> Result<Value, DxrError>;
}

#[derive(Debug, Error)]
/// error type used for conversion errors between XML-RPC values and Rust values
pub enum DxrError {
    /// error variant describing XML parser errors
    #[error("Failed to parse XML data: {}", .error)]
    InvalidData {
        /// description of the parsing error
        error: String,
    },
    /// error variant describing a missing struct field
    #[error("Missing struct field: {}", .name)]
    MissingField {
        /// name of the missing struct field
        name: Cow<'static, str>,
    },
    /// error variant describing mismatched return types
    #[error("Type mismatch: got {} values, expected {}", .argument, .expected)]
    ReturnMismatch {
        /// number of returned values
        argument: usize,
        /// number of expected values
        expected: usize,
    },
    /// error variant describing a server fault
    #[error("{}", .fault)]
    ServerFault {
        /// fault data returned by the server
        fault: Fault,
    },
    /// error variant describing a type mismatch between XML-RPC value and a Rust struct field
    #[error("Type mismatch: got {}, expected {}", .argument, .expected)]
    WrongType {
        /// mismatched input type
        argument: Cow<'static, str>,
        /// expected input type
        expected: Cow<'static, str>,
    },
}

impl DxrError {
    /// construct a [`DxrError`] for invalid input data
    pub fn invalid_data(error: String) -> DxrError {
        DxrError::InvalidData { error }
    }

    /// construct a [`DxrError`] for a missing struct field
    pub fn missing_field(name: &'static str) -> DxrError {
        DxrError::MissingField {
            name: Cow::Borrowed(name),
        }
    }

    /// construct a [`DxrError`] for unexpected number of returned values
    pub fn return_mismatch(argument: usize, expected: usize) -> DxrError {
        DxrError::ReturnMismatch { argument, expected }
    }

    /// construct a [`DxrError`] for a server fault
    pub fn server_fault(fault: Fault) -> DxrError {
        DxrError::ServerFault { fault }
    }

    /// construct a [`DxrError`] for a type mismatch
    pub fn wrong_type(argument: &'static str, expected: &'static str) -> DxrError {
        DxrError::WrongType {
            argument: Cow::Borrowed(argument),
            expected: Cow::Borrowed(expected),
        }
    }
}

#[cfg(test)]
mod checks;

#[cfg(test)]
mod tests;
