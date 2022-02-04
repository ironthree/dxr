# DXR: declarative XML-RPC (shared traits, types, and implementations)

[![crates.io](https://img.shields.io/crates/v/dxr_shared.svg)](https://crates.io/crates/dxr_shared/)
[![crates.io](https://img.shields.io/crates/d/dxr_shared.svg)](https://crates.io/crates/dxr_shared/)
[![crates.io](https://img.shields.io/crates/l/dxr_shared.svg)](https://crates.io/crates/dxr_shared/)
[![docs.rs](https://docs.rs/dxr_shared/badge.svg)](https://docs.rs/dxr_shared/)

The dxr project provides crates for writing XML-RPC API clients and servers in Rust.

This crate contains definitions of all data types, type conversion functionality, and
(de)serialization implementations that are needed for dealing with XML-RPC values of
all kinds.

All public items are re-exported from the `dxr` crate, so this crate should be considered
an internal implementation detail, and never be imported or used directly.
