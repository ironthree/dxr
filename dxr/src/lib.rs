//! # dxr: declarative xml-rpc
//!
//! The `dxr` crate provides types, macros, and other functionality which can be used to write
//! fast and correct XML-API clients and servers in Rust conveniently.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

#[cfg(feature = "derive")]
pub use dxr_derive::{FromDXR, ToDXR};

pub use dxr_shared::{DxrError, FromDXR, ToDXR, ToParams, Value, XML_RPC_DATE_FORMAT};

// re-export of chrono, since DateTime / Utc are part of the public API
pub use dxr_shared::chrono;
// re-export of url::Url, since it is part of the public API
pub use url;

mod call;
pub use call::*;

mod client;
pub use client::*;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::*;

#[cfg(test)]
mod tests;
