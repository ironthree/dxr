# DXR: declarative XML-RPC (client implementation)

[![crates.io](https://img.shields.io/crates/v/dxr_client.svg)](https://crates.io/crates/dxr_client/)
[![crates.io](https://img.shields.io/crates/d/dxr_client.svg)](https://crates.io/crates/dxr_client/)
[![crates.io](https://img.shields.io/crates/l/dxr_client.svg)](https://crates.io/crates/dxr_client/)
[![docs.rs](https://docs.rs/dxr_client/badge.svg)](https://docs.rs/dxr_client/)

The `dxr` project provides crates for writing XML-RPC clients and servers in Rust.

This crate contains a building blocks for writing XML-RPC clients based on `dxr`.

It also includes an implementation of an `async` XML-RPC client using `reqwest`, which is disabled
by default. To enable the `reqwest` support, enable the `"reqwest"` feature of this crate.

To enable convenience functionality for "system.multicall" support, enable the `multicall` feature.
