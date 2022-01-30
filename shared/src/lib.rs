//! # dxr_shared
//!
//! This crate provides base implementations of all XML-RPC types and functionality that is used in
//! the macros provided by `dxr_derive` and the high-level functionality provided in `dxr` itself.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

/// date & time format used by the XML-RPC `dateTime.iso8601` value type
pub const XML_RPC_DATE_FORMAT: &str = "%Y%m%dT%H:%M:%S";

mod ser_de;

mod impls;
pub use impls::*;

mod types;
pub use types::*;

/// conversion trait from XML-RPC values to primitives, `Option`, `HashMap`, and user-defined types
pub trait FromValue<T> {
    /// fallible conversion method from an XML-RPC value into the target type
    ///
    /// If the value contains a type that is not compatible with the target type, the conversion
    /// will fail.
    fn from_value(value: &Value) -> Result<T, ()>;
}

#[cfg(test)]
mod tests;
