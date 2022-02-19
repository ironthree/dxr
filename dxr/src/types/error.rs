use std::borrow::Cow;

use thiserror::Error;

use crate::types::fault::Fault;

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
    #[error("Struct '{}' missing field: {}", .name, .field)]
    MissingField {
        /// name of the struct that has a missing field
        name: Cow<'static, str>,
        /// name of the missing struct field
        field: Cow<'static, str>,
    },
    #[error("Parameter mismatch: got {} values, expected {}", .argument, .expected)]
    /// error variant describing value number mismatch
    ParameterMismatch {
        /// number of received values
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

    /// check if a given [`DxrError`] was raised for invalid data
    pub fn is_invalid_data(&self) -> bool {
        matches!(self, DxrError::InvalidData { .. })
    }

    /// check for [`DxrError::InvalidData`] and return the inner error in case of a match
    ///
    /// The returned string describes the XML (de)serialization issue.
    pub fn as_invalid_data(&self) -> Option<&str> {
        if let DxrError::InvalidData { error } = self {
            Some(error)
        } else {
            None
        }
    }

    /// construct a [`DxrError`] for a missing struct field
    pub fn missing_field(name: &'static str, field: &'static str) -> DxrError {
        DxrError::MissingField {
            name: Cow::Borrowed(name),
            field: Cow::Borrowed(field),
        }
    }

    /// check if a given [`DxrError`] was raised for a missing struct field
    pub fn is_missing_field(&self) -> bool {
        matches!(self, DxrError::MissingField { .. })
    }

    /// check for [`DxrError::MissingField`] and return the inner error in case of a match
    ///
    /// The returned value is a tuple of (struct name, missing field name).
    pub fn as_missing_field(&self) -> Option<(&str, &str)> {
        if let DxrError::MissingField { name, field } = self {
            Some((name, field))
        } else {
            None
        }
    }

    /// construct a [`DxrError`] for a parameter number mismatch
    pub fn parameter_mismatch(argument: usize, expected: usize) -> DxrError {
        DxrError::ParameterMismatch { argument, expected }
    }

    /// check if a given [`DxrError`] was raised for unexpected number of return values
    pub fn is_parameter_mismatch(&self) -> bool {
        matches!(self, DxrError::ParameterMismatch { .. })
    }

    /// check for [`DxrError::ParameterMismatch`] and return the inner error in case of a match
    ///
    /// The returned value is a tuple of the numbers of (received arguments, expected arguments).
    pub fn as_parameter_mismatch(&self) -> Option<(usize, usize)> {
        if let DxrError::ParameterMismatch { argument, expected } = self {
            Some((*argument, *expected))
        } else {
            None
        }
    }

    /// construct a [`DxrError`] for a server fault
    pub fn server_fault(fault: Fault) -> DxrError {
        DxrError::ServerFault { fault }
    }

    /// check if a given [`DxrError`] was raised for a server fault
    pub fn is_server_fault(&self) -> bool {
        matches!(self, DxrError::ServerFault { .. })
    }

    /// check for [`DxrError::ServerFault`] and return the inner error in case of a match
    ///
    /// The returned value is a reference to the inner [`Fault`].
    pub fn as_server_fault(&self) -> Option<&Fault> {
        if let DxrError::ServerFault { fault } = self {
            Some(fault)
        } else {
            None
        }
    }

    /// construct a [`DxrError`] for a type mismatch
    pub fn wrong_type(argument: &'static str, expected: &'static str) -> DxrError {
        DxrError::WrongType {
            argument: Cow::Borrowed(argument),
            expected: Cow::Borrowed(expected),
        }
    }

    /// check if a given [`DxrError`] was raised for a type mismatch
    pub fn is_wrong_type(&self) -> bool {
        matches!(self, DxrError::WrongType { .. })
    }

    /// check for [`DxrError::WrongType`] and return the inner error in case of a match
    ///
    /// The returned value is a tuple of the names of (received type, expected type).
    pub fn as_wrong_type(&self) -> Option<(&str, &str)> {
        if let DxrError::WrongType { argument, expected } = self {
            Some((argument, expected))
        } else {
            None
        }
    }
}

impl From<DxrError> for Fault {
    fn from(error: DxrError) -> Self {
        match error {
            DxrError::InvalidData { .. } => Fault::new(400, error.to_string()),
            DxrError::MissingField { .. } => Fault::new(400, error.to_string()),
            DxrError::ParameterMismatch { .. } => Fault::new(400, error.to_string()),
            DxrError::ServerFault { fault } => fault,
            DxrError::WrongType { .. } => Fault::new(400, error.to_string()),
        }
    }
}
