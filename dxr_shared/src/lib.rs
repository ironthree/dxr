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
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # dxr: Declarative XML-RPC
//!
//! The `dxr` crate provides types, macros, and other functionality which can be used to write
//! fast and correct XML-RPC clients and servers in Rust conveniently.
//!
//! The APIs for implementing both clients and servers are designed to require no boilerplate code
//! (outside this crate, that is), and implements type conversions from Rust to XML-RPC types
//! automatically for all supported data types. Custom struct types are also supported, if they
//! derive or manually implement the [`FromDXR`] and / or [`ToDXR`] traits.
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
//! The APIs for setting up an XML-RPC server are intended to be similarly straight-forward,
//! and allow embedding the XML-RPC server endpoint route into other servers. First, set up a
//! [`RouteBuilder`], set up all method handlers, build it into an [`axum::Router`], and then
//! either use this route as part of a larger server, or create a standalone service from it.
//!
//! ```
//! # #[cfg(feature = "axum-server")] {
//! use dxr::RouteBuilder;
//! let route = RouteBuilder::new().build();
//! # }
//! ```
//!
//! Now, this is not a very useful XML-RPC endpoint, since it does not know about any method calls.
//! An arbitrary number of method handlers can be registered with the [`RouteBuilder`] before
//! building the [`axum::Router`].
//!
//! ```
//! # #[cfg(feature = "axum-server")] {
//! use dxr::axum::http::HeaderMap;
//! use dxr::{Fault, FromParams, HandlerFn, RouteBuilder, ToDXR, Value};
//!
//! fn hello_handler(params: &[Value], _headers: &HeaderMap) -> Result<Option<Value>, Fault> {
//!     let name = String::from_params(params)?;
//!     Ok(Some(format!("Handler function says: Hello, {}!", name).to_dxr()?))
//! }
//!
//! let route = RouteBuilder::new()
//!     .set_path("/")
//!     .add_method("hello", Box::new(hello_handler as HandlerFn))
//!     .build();
//! # }
//! ```
//!
//! Method handlers must either implement [`Handler`] themselves, or align with the [`HandlerFn`]
//! function pointer type, for which this trait implementation is already provided.
//!
//! Using this route in a standalone server with only an XML-RPC endpoint is straightforward:
//!
//! ```no_run
//! # #[cfg(feature = "axum-server")] {
//! # tokio_test::block_on(async {
//! # use dxr::RouteBuilder;
//! # let route = RouteBuilder::new().build();
//! use dxr::Server;
//!
//! let server = Server::from_route("0.0.0.0:3000".parse().unwrap(), route);
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
//! By default, only the basic client and server support functionality and derive macros are
//! enabled. All features can also be enabled individually -- by turning off default features and
//! just enabling the required features.
//!
//! Client and server functionality are both optional, since they pull in additional dependencies.
//! The features can be enabled and disabled separately, but having neither of the two features
//! enabled makes little sense, as it disables most of the crate's functionality. There is
//! additional support functionality for servers that use [`axum`] and [`tokio`], which can be
//! enabled with the `axum-server` feature.
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
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use dxr_derive::{FromDXR, ToDXR};

// re-export chrono: DateTime / Utc are part of the public API
pub use chrono;

mod error;
pub use error::*;

mod fault;
pub use fault::*;

mod impls;
pub use impls::*;

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
