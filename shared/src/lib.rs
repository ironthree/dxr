//! # dxr_shared
//!
//! This crate provides base implementations of all XML-RPC types and functionality that is used in
//! the macros provided by `dxr_derive` and the high-level functionality provided in `dxr` itself.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

// re-export of chrono, since it is part of the public API
pub use chrono;

mod ser_de;

mod error;
pub use error::*;

mod from;
pub use from::*;

mod params;
pub use params::*;

mod to;
pub use to::*;

mod traits;
pub use traits::*;

pub mod types;

/// date & time format used by the XML-RPC `dateTime.iso8601` value type
pub const XML_RPC_DATE_FORMAT: &str = "%Y%m%dT%H:%M:%S";

#[cfg(test)]
mod checks;

#[cfg(test)]
mod tests;
