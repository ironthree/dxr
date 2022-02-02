# DXR: declarative XML-RPC

This repository contains work-in-progress crates for writing XML-RPC API clients
and servers in Rust.

- `dxr`: high-level client and server implementations
- `dxr_derive`: `ToDXR` and `FromDXR` derive macros for custom structs
- `dxr_shared`: definition of conversion traits and XML-RPC types for (de)serialization  

All relevant parts of `dxr_derive` and `dxr_shared` are re-exported in the `dxr` crate,
they are not supposed to be used directly.

