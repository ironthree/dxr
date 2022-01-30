//! # dxr: declarative xml-rpc
//!
//! The `dxr` crate provides types, macros, and other functionality which can be used to write
//! fast and correct XML-API clients in Rust conveniently.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

pub use dxr_derive::{FromDXR, ToDXR};

pub use dxr_shared::{types, FromDXR, ToDXR, ValueError, XML_RPC_DATE_FORMAT};

/// re-export of chrono, since it is part of the public API
pub use dxr_shared::chrono;

#[cfg(test)]
mod tests;
