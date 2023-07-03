use thiserror::Error;

use crate::error::DxrError;
use crate::traits::TryFromValue;
use crate::values::FaultResponse;

/// XML-RPC server fault (consisting of a numeric error code and a message)
///
/// *Note*: There are no standardized numeric error codes, and they will likely be
/// specific to the server application.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Server Fault {}: {}", .code, .string)]
pub struct Fault {
    code: i32,
    string: String,
}

impl Fault {
    /// Construct a new [`Fault`] from numeric error code and an error message.
    pub fn new(code: i32, string: String) -> Fault {
        Fault { code, string }
    }

    /// Retrieve the numeric error code from the [`Fault`].
    pub fn code(&self) -> i32 {
        self.code
    }

    /// Retrieve the error message from the [`Fault`].
    pub fn string(&self) -> &str {
        self.string.as_str()
    }
}

impl TryFrom<FaultResponse> for Fault {
    type Error = DxrError;

    fn try_from(value: FaultResponse) -> Result<Self, Self::Error> {
        let mut members = value.members().iter();

        let (first, second) = match (members.next(), members.next(), members.next()) {
            (Some(first), Some(second), None) => (first, second),
            _ => return Err(DxrError::parameter_mismatch(members.len(), 2)),
        };

        let fault_code = if first.name() == "faultCode" {
            first.inner()
        } else {
            return Err(DxrError::missing_field("fault", "faultCode"));
        };

        let fault_string = if second.name() == "faultString" {
            second.inner()
        } else {
            return Err(DxrError::missing_field("fault", "faultString"));
        };

        let code: i32 = i32::try_from_value(fault_code)?;
        let string: String = String::try_from_value(fault_string)?;

        Ok(Fault::new(code, string))
    }
}
