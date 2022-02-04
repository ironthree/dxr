//! # dxr: declarative xml-rpc
//!
//! The `dxr` crate provides types, macros, and other functionality which can be used to write
//! fast and correct XML-API clients and servers in Rust conveniently.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

#[cfg(feature = "derive")]
pub use dxr_derive::{FromDXR, ToDXR};

pub use dxr_shared::{DxrError, Fault, FromDXR, FromParams, ToDXR, ToParams, Value, XML_RPC_DATE_FORMAT};

// re-export chrono: DateTime / Utc are part of the public API
pub use dxr_shared::chrono;

// re-export url::Url: it is part of the public client API
#[cfg(feature = "client")]
pub use url;

// client-specific modules
#[cfg(feature = "client")]
mod call;
#[cfg(feature = "client")]
pub use call::*;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::*;

// server-specific modules
#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::*;

#[cfg(test)]
mod tests;
