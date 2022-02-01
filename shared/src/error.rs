use std::borrow::Cow;

use thiserror::Error;

use crate::types::Fault;

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
