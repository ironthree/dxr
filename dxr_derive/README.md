# DXR: declarative XML-RPC (derive macros)

[![crates.io](https://img.shields.io/crates/v/dxr_derive.svg)](https://crates.io/crates/dxr_derive/)
[![crates.io](https://img.shields.io/crates/d/dxr_derive.svg)](https://crates.io/crates/dxr_derive/)
[![crates.io](https://img.shields.io/crates/l/dxr_derive.svg)](https://crates.io/crates/dxr_derive/)
[![docs.rs](https://docs.rs/dxr_derive/badge.svg)](https://docs.rs/dxr_derive/)

The dxr project provides crates for writing XML-RPC API clients and servers in Rust.

This crate contains implementations of derive macros for the `TryFromValue` and `TryToValue` traits.
They are re-exported from the `dxr` crate when the `derive` feature is enabled, so this crate should
be considered an internal implementation detail, and never be imported or used directly.
