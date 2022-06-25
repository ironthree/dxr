# DXR: declarative XML-RPC (client implementation)

[![crates.io](https://img.shields.io/crates/v/dxr_client.svg)](https://crates.io/crates/dxr_client/)
[![crates.io](https://img.shields.io/crates/d/dxr_client.svg)](https://crates.io/crates/dxr_client/)
[![crates.io](https://img.shields.io/crates/l/dxr_client.svg)](https://crates.io/crates/dxr_client/)
[![docs.rs](https://docs.rs/dxr_client/badge.svg)](https://docs.rs/dxr_client/)

The dxr project provides crates for writing XML-RPC API clients and servers in Rust.

This crate contains an implementation of an `async` XML-RPC client using `reqwest`. The client
functionality is re-exported from the `dxr` crate when the `client` feature is enabled, so
this crate should be considered an internal implementation detail, and never be imported or
used directly.
