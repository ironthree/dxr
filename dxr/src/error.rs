use std::borrow::Cow;

use thiserror::Error;

use crate::fault::Fault;

#[derive(Debug, Error)]
/// Error type representing conversion errors between XML-RPC values and Rust values.
pub enum DxrError {
    /// Error variant for XML parser errors.
    #[error("Failed to parse XML data: {}", .error)]
    InvalidData {
        /// description of the parsing error
        error: String,
    },
    /// Error variant for a missing struct field.
    #[error("Struct '{}' missing field: {}", .name, .field)]
    MissingField {
        /// name of the struct that has a missing field
        name: Cow<'static, str>,
        /// name of the missing struct field
        field: Cow<'static, str>,
    },
    #[error("Parameter mismatch: got {} values, expected {}", .argument, .expected)]
    /// Error variant for mismatch with an expected number of values.
    ParameterMismatch {
        /// number of received values
        argument: usize,
        /// number of expected values
        expected: usize,
    },
    /// Error variant for mismatch with an expected value type.
    #[error("Type mismatch: got {}, expected {}", .argument, .expected)]
    WrongType {
        /// mismatched input type
        argument: Cow<'static, str>,
        /// expected input type
        expected: Cow<'static, str>,
    },
}

impl DxrError {
    /// Construct a [`DxrError`] for invalid input data.
    pub fn invalid_data(error: String) -> DxrError {
        DxrError::InvalidData { error }
    }

    /// Check if a given [`DxrError`] was raised for invalid data.
    pub fn is_invalid_data(&self) -> bool {
        matches!(self, DxrError::InvalidData { .. })
    }

    /// Check for [`DxrError::InvalidData`] and return the inner error in case of a match.
    ///
    /// The returned string describes the XML (de)serialization issue.
    pub fn as_invalid_data(&self) -> Option<&str> {
        if let DxrError::InvalidData { error } = self {
            Some(error)
        } else {
            None
        }
    }

    /// Construct a [`DxrError`] for a missing struct field.
    pub fn missing_field(name: &'static str, field: &'static str) -> DxrError {
        DxrError::MissingField {
            name: Cow::Borrowed(name),
            field: Cow::Borrowed(field),
        }
    }

    /// Check if a given [`DxrError`] was raised for a missing struct field.
    pub fn is_missing_field(&self) -> bool {
        matches!(self, DxrError::MissingField { .. })
    }

    /// Check for [`DxrError::MissingField`] and return the inner error in case of a match.
    ///
    /// The returned value is a tuple of (struct name, missing field name).
    pub fn as_missing_field(&self) -> Option<(&str, &str)> {
        if let DxrError::MissingField { name, field } = self {
            Some((name, field))
        } else {
            None
        }
    }

    /// Construct a [`DxrError`] for a parameter number mismatch.
    pub fn parameter_mismatch(argument: usize, expected: usize) -> DxrError {
        DxrError::ParameterMismatch { argument, expected }
    }

    /// Check if a given [`DxrError`] was raised for unexpected number of return values.
    pub fn is_parameter_mismatch(&self) -> bool {
        matches!(self, DxrError::ParameterMismatch { .. })
    }

    /// Check for [`DxrError::ParameterMismatch`] and return the inner error in case of a match.
    ///
    /// The returned value is a tuple of the numbers of (received arguments, expected arguments).
    pub fn as_parameter_mismatch(&self) -> Option<(usize, usize)> {
        if let DxrError::ParameterMismatch { argument, expected } = self {
            Some((*argument, *expected))
        } else {
            None
        }
    }

    /// Construct a [`DxrError`] for a type mismatch.
    pub fn wrong_type(argument: &'static str, expected: &'static str) -> DxrError {
        DxrError::WrongType {
            argument: Cow::Borrowed(argument),
            expected: Cow::Borrowed(expected),
        }
    }

    /// Check if a given [`DxrError`] was raised for a type mismatch.
    pub fn is_wrong_type(&self) -> bool {
        matches!(self, DxrError::WrongType { .. })
    }

    /// Check for [`DxrError::WrongType`] and return the inner error in case of a match.
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
            DxrError::WrongType { .. } => Fault::new(400, error.to_string()),
        }
    }
}
