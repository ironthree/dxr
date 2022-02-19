mod ser_de;

mod dxr;
pub use dxr::*;

mod error;
pub use error::*;

mod fault;
pub use fault::*;

mod params;
pub use params::*;

mod traits;
pub use traits::*;

mod structs;
pub use structs::{FaultResponse, MethodCall, MethodResponse, Value};

mod util;

/// date & time format used by the XML-RPC `dateTime.iso8601` value type
pub const XML_RPC_DATE_FORMAT: &str = "%Y%m%dT%H:%M:%S";

// property-based (de)serialization tests
#[cfg(test)]
mod checks;

// standard (de)serialization tests
#[cfg(test)]
mod tests;
