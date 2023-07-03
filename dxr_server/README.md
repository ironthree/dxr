# DXR: declarative XML-RPC (server implementation)

[![crates.io](https://img.shields.io/crates/v/dxr_server.svg)](https://crates.io/crates/dxr_server/)
[![crates.io](https://img.shields.io/crates/d/dxr_server.svg)](https://crates.io/crates/dxr_server/)
[![crates.io](https://img.shields.io/crates/l/dxr_server.svg)](https://crates.io/crates/dxr_server/)
[![docs.rs](https://docs.rs/dxr_server/badge.svg)](https://docs.rs/dxr_server/)

The `dxr` project provides crates for writing XML-RPC clients and servers in Rust.

This crate contains a building blocks for writing XML-RPC servers based on `dxr`.

It also includes a complete XML-RPC server implementation based on the `axum` web framework, which
is disabled by default. To enable the `axum` support, enable the `"axum"` feature of this crate.
