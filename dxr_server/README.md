# DXR: declarative XML-RPC (server implementation)

[![crates.io](https://img.shields.io/crates/v/dxr_server.svg)](https://crates.io/crates/dxr_server/)
[![crates.io](https://img.shields.io/crates/d/dxr_server.svg)](https://crates.io/crates/dxr_server/)
[![crates.io](https://img.shields.io/crates/l/dxr_server.svg)](https://crates.io/crates/dxr_server/)
[![docs.rs](https://docs.rs/dxr_server/badge.svg)](https://docs.rs/dxr_server/)

The dxr project provides crates for writing XML-RPC API clients and servers in Rust.

This crate contains a basic implementation of an `async` XML-RPC server. The server functionality
is re-exported from the `dxr` crate when the `server` feature is enabled, so this crate should be
considered an internal implementation detail, and never be imported or used directly.
