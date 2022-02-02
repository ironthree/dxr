use std::fmt::{Display, Formatter};

use crate::types::{FaultResponse, Type};

/// data type representing an XML-RPC server fault (numeric error code and message)
#[derive(Clone, Debug, PartialEq)]
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

impl Display for Fault {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Fault {}: {}", self.code, self.string)
    }
}

impl From<FaultResponse> for Fault {
    fn from(f: FaultResponse) -> Self {
        let members = f.members();

        let first = members.get(0).unwrap();
        let second = members.get(1).unwrap();

        match (first.inner().inner(), second.inner().inner()) {
            (Type::Integer(code), Type::String(string)) => Fault {
                code: *code,
                string: string.clone(),
            },
            (Type::String(string), Type::Integer(code)) => Fault {
                code: *code,
                string: string.clone(),
            },
            _ => unreachable!(),
        }
    }
}
