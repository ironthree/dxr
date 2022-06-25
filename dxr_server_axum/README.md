# DXR: declarative XML-RPC (axum server)

[![crates.io](https://img.shields.io/crates/v/dxr_server_axum.svg)](https://crates.io/crates/dxr_server_axum/)
[![crates.io](https://img.shields.io/crates/d/dxr_server_axum.svg)](https://crates.io/crates/dxr_server_axum/)
[![crates.io](https://img.shields.io/crates/l/dxr_server_axum.svg)](https://crates.io/crates/dxr_server_axum/)
[![docs.rs](https://docs.rs/dxr_server_axum/badge.svg)](https://docs.rs/dxr_server_axum/)

The dxr project provides crates for writing XML-RPC API clients and servers in Rust.

This crate contains an implementation of an `async` XML-RPC server using `axum`. This functionality
is re-exported from the `dxr` crate when the `server-axum` feature is enabled, so this crate should be
considered an internal implementation detail, and never be imported or used directly.
