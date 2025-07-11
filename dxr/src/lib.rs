#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! # dxr
//!
//! This crate provides an implementation of XML-RPC types, (de)serialization support, and
//! conversion between XML-RPC values and Rust values (with optional support for non-standard `i8`
//! and `nil` types, which can be enabled with the respective feature flags).
//!
//! Support for writing XML-RPC clients / servers is provided in separate crates:
//!
//! - `dxr_client`: generic XML-RPC client code and a default implementation based on `reqwest`
//! - `dxr_server`: generic XML-RPC server code and a default implementation based on `axum`
//!
//! The table below lists XML-RPC types and their equivalent Rust types.
//!
//! | XML-RPC value type | Rust type                 |
//! |: ----------------- |: ------------------------ |
//! | `i4`               | [`i32`]                   |
//! | `i8`               | [`i64`]                   |
//! | `boolean`          | [`bool`]                  |
//! | `string`           | [`String`] / [`&str`]     |
//! | `double`           | [`f64`]                   |
//! | `dateTime.iso8601` | [`chrono::NaiveDateTime`] |
//! | `base64`           | [`Vec<u8>`]               |
//! | `nil`              | [`Option<T>`]             |
//!
//! Additionally, the [`TryFromValue`] and [`TryToValue`] traits (which implement the conversion
//! between XML-RPC value types and Rust types) are implemented for
//!
//! - [`Vec<T>`], slices `&[T]`, and fixed-size arrays `[T; N]`,
//! - smart pointer types like [`Box<T>`], [`Cow<T>`], [`Rc<T>`], and [`Arc<T>`],
//! - mappings like [`HashMap<String, T>`] / [`HashMap<&str, T>`],
//! - tuples `(T, ...)` with up to eight members
//!
//! (as long as the inner type `T` also implement these traits).
//!
//! ## Features
//!
//! This crate provides optional features, all of which are disabled by default:
//!
//! - `derive`: include procedural macros for deriving the [`TryFromValue`] and [`TryToValue`]
//!   traits for custom structs
//! - `i8`: enable support for the non-standard `i8` value type
//! - `nil`: enable support for the non-standard `nil` value type

// imports for intra-doc links
#[cfg(doc)]
use std::{borrow::Cow, collections::HashMap, rc::Rc, sync::Arc};

#[cfg(feature = "derive")]
pub use dxr_derive::{TryFromValue, TryToValue};

mod base64;

mod error;
pub use error::*;

mod fault;
pub use fault::*;

mod impls;

#[cfg(feature = "multicall")]
mod multicall;
#[cfg(feature = "multicall")]
pub use multicall::*;

mod traits;
pub use traits::*;

mod values;
pub use values::*;

mod xml;
pub use xml::*;

// property-based tests
#[cfg(test)]
mod checks;

// standard tests
#[cfg(test)]
mod tests;
