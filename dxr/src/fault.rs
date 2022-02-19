use thiserror::Error;

use crate::error::DxrError;
use crate::traits::FromDXR;
use crate::values::FaultResponse;

/// XML-RPC server fault (numeric error code and message)
#[derive(Clone, Debug, Error, PartialEq)]
#[error("Server Fault {}: {}", .code, .string)]
pub struct Fault {
    code: i32,
    string: String,
}

impl Fault {
    /// constructor for a new [`Fault`]
    pub fn new(code: i32, string: String) -> Fault {
        Fault { code, string }
    }

    /// error code associated with the fault
    pub fn code(&self) -> i32 {
        self.code
    }

    /// error message associated with the fault
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

        let code: i32 = i32::from_dxr(fault_code)?;
        let string: String = String::from_dxr(fault_string)?;

        Ok(Fault::new(code, string))
    }
}
