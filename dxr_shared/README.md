# DXR: declarative XML-RPC (shared implementation details)

[![crates.io](https://img.shields.io/crates/v/dxr_shared.svg)](https://crates.io/crates/dxr_shared/)
[![crates.io](https://img.shields.io/crates/d/dxr_shared.svg)](https://crates.io/crates/dxr_shared/)
[![crates.io](https://img.shields.io/crates/l/dxr_shared.svg)](https://crates.io/crates/dxr_shared/)
[![docs.rs](https://docs.rs/dxr_shared/badge.svg)](https://docs.rs/dxr_shared/)

The dxr project provides crates for writing XML-RPC API clients and servers in Rust.

This crate contains implementations of type conversions between XML strings, XML-RPC values, and
the corresponding Rust types. This functionality should be considered an internal implementation
detail, and never be imported or used directly.
