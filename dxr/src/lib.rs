#![deny(unsafe_code)]
#![warn(explicit_outlives_requirements)]
#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # dxr: Declarative XML-RPC
//!
//! The `dxr` crate provides types, macros, and other functionality which can be used to write
//! fast and correct XML-RPC clients and servers in Rust conveniently.
//!
//! The APIs for implementing both clients and servers are designed to require no boilerplate code
//! (outside this crate, that is), and implements type conversions from Rust to XML-RPC types
//! automatically for all supported data types. Custom struct types are also supported, if they
//! derive the [`FromDXR`] and / or [`ToDXR`] traits.
//!
//! ## Client interface
//!
//! A new XML-RPC client is initialized by creating a [`ClientBuilder`] instance for a specific
//! XML-RPC server URL, modifying it with custom settings, and then building it into a [`Client`].
//!
//! ```
//! # #[cfg(feature = "client")] {
//! use dxr::{Client, ClientBuilder};
//! use url::Url;
//!
//! let url = Url::parse("https://example.com/xml-rpc/").unwrap();
//! let client: Client = ClientBuilder::new(url).user_agent("dxr-client-example").build();
//! # }
//! ```
//!
//! This client can then be used to issue Remote Procedure [`Call`]s:
//!
//! ```no_run
//! # #[cfg(feature = "client")] {
//! # use dxr::{Client, ClientBuilder};
//! # use url::Url;
//! # let url = Url::parse("https://example.com/xml-rpc/").unwrap();
//! # let client: Client = ClientBuilder::new(url).user_agent("dxr-client-example").build();
//! # tokio_test::block_on(async {
//! use dxr::Call;
//!
//! // create an RPC request with one string argument and an expected string return value
//! let request = Call::new("hello", "DXR");
//! let result: String = client.call(request).await.unwrap();
//! # })
//! # }
//! ```
//!
//! The `examples/client.rs` file contains a complete implementation of a simple client binary,
//! which can be used to issue an RPC request to the server provided by the server example.
//!
//! ## Server interface
//!
//! The APIs for setting up an XML-RPC server are intended to be similarly straight-forward. First,
//! set up a [`ServerBuilder`] by giving it the socket address it should bind to, setting up all
//! method handlers, and then building it into a [`Server`]:
//!
//! ```
//! # #[cfg(feature = "server")] {
//! use dxr::ServerBuilder;
//! let server = ServerBuilder::new("0.0.0.0:3000".parse().unwrap()).build();
//! # }
//! ```
//!
//! Now, this is not a very useful server, since it does not know about any method calls. An
//! arbitrary number of method handlers can be registered with the [`ServerBuilder`] before building
//! the [`Server`].
//!
//! ```
//! # #[cfg(feature = "server")] {
//! use dxr::axum::http::HeaderMap;
//! use dxr::{Fault, FromParams, HandlerFn, ToDXR, Value};
//!
//! fn hello_handler(params: &[Value], _headers: &HeaderMap) -> Result<Value, Fault> {
//!     let name = String::from_params(params)?;
//!     Ok(format!("Handler function says: Hello, {}!", name).to_dxr()?)
//! }
//!
//! use dxr::ServerBuilder;
//! let server = ServerBuilder::new("0.0.0.0:3000".parse().unwrap())
//!     .add_method("hello", Box::new(hello_handler as HandlerFn))
//!     .build();
//! # }
//! ```
//!
//! Method handlers must either implement [`Handler`] themselves, or align with the [`HandlerFn`]
//! function pointer type, for which this trait implementation is already provided.
//!
//! Finally, call the [`Server::serve`] method to accept and handle requests:
//!
//! ```no_run
//! # #[cfg(feature = "server")] {
//! # tokio_test::block_on(async {
//! # use dxr::ServerBuilder;
//! # let server = ServerBuilder::new("0.0.0.0:3000".parse().unwrap()).build();
//! server.serve().await.unwrap();
//! # })
//! # }
//! ```
//!
//! The `examples/server.rs` file contains an implementation of a simple server binary, which
//! provides a `hello(String)` method that returns a welcome message, and a `countme()` method that
//! returns the number of times the `countme()` method has been called since the server was started.
//!
//! ## Optional Features
//!
//! By default, all features except the `server` feature are enabled. All features can also be
//! enabled individually -- by turning off default features and just enabling the required features.
//!
//! Client and server functionality are both optional, since they pull in additional dependencies.
//! The features can be enabled and disabled separately, though the `client` feature is enabled by
//! default, and having neither of the two features enabled makes little sense, as it disabled most
//! of the crate's functionality. There is additional support functionality for servers that use
//! [`tokio`], which can be enabled with the `tokio` feature.
//!
//! This crates also supports deriving conversion trait implementations for custom, user-defined
//! structs. The derive macros are available if the `derive` feature is enabled (which it is by
//! default).
//!
//! There is also optional support for two non-standard XML-RPC extensions:
//!
//! - long integers (`<i8>`): mapped to [`i64`], enabled with the `i8` feature
//! - null values (`<nil/>`): mapped to [`Option`]`<T>`, enabled with the `nil` feature

#[cfg(feature = "derive")]
#[doc(inline)]
pub use dxr_derive::{FromDXR, ToDXR};

#[doc(inline)]
pub use dxr_shared::{DxrError, Fault, FromDXR, FromParams, ToDXR, ToParams, Value, XML_RPC_DATE_FORMAT};

// re-export chrono: DateTime / Utc are part of the public API
pub use dxr_shared::chrono;

// re-export url: public client API
#[cfg(feature = "client")]
pub use url;

// re-export async-trait: public server API
#[cfg(feature = "server")]
pub use async_trait;

// re-export axum: public server API
#[cfg(feature = "server")]
pub use axum;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::*;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::*;

#[cfg(test)]
mod tests;
