#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! # dxr_client
//!
//! This crate provides generic XML-RPC client functionality based on [`dxr`].

#[cfg(feature = "reqwest")]
mod reqwest_support;
#[cfg(feature = "reqwest")]
pub use reqwest_support::*;

// re-export url::URL, as it is exposed in the the public API
#[cfg(feature = "reqwest")]
pub use url::Url;

/// default value of the `User-Agent` HTTP header for XML-RPC requests
pub const DEFAULT_USER_AGENT: &str = concat!("dxr-client-v", env!("CARGO_PKG_VERSION"));
