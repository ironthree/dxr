#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![deny(unsafe_code)]
#![warn(explicit_outlives_requirements)]
#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unreachable_pub)]
#![warn(clippy::unwrap_used)]
#![deprecated(
    since = "0.5.5",
    note = "The dxr_shared crate was renamed to dxr with version 0.6.0."
)]

//! # dxr_shared
//!
//! This crate is an implementation detail of the `dxr` crate, which provides the implementation of
//! XML-RPC types, (de)serialization support, and conversion between XML-RPC values and Rust values.

// re-export chrono: DateTime / Utc are part of the public API
pub use chrono;

mod error;
pub use error::*;

mod fault;
pub use fault::*;

mod impls;

mod traits;
pub use traits::*;

mod values;
pub use values::*;

// property-based tests
#[cfg(test)]
mod checks;

// standard tests
#[cfg(test)]
mod tests;
