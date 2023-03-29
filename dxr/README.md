# DXR: declarative XML-RPC

[![crates.io](https://img.shields.io/crates/v/dxr.svg)](https://crates.io/crates/dxr/)
[![crates.io](https://img.shields.io/crates/d/dxr.svg)](https://crates.io/crates/dxr/)
[![crates.io](https://img.shields.io/crates/l/dxr.svg)](https://crates.io/crates/dxr/)
[![docs.rs](https://docs.rs/dxr/badge.svg)](https://docs.rs/dxr/)

The dxr project provides crates for writing XML-RPC API clients and servers in Rust.
The goal is to match the [XML-RPC Specification](http://xmlrpc.com/spec.md) -- even
though some parts of it are under-specified -- and provide optional support for some
common non-standard extensions.

Documentation of the public API and a tutorial-style introduction are available on
the [docs.rs](https://docs.rs/dxr/) page for this crate. Additionally, there are a few
example binaries in `dxr/examples`.

## Features

- (de)serialization support for converting XML-RPC XML strings into strongly-typed Rust values
- conversion traits between XML-RPC values and Rust primitives, arrays, slices, byte arrays,
  tuples, hashmaps, and custom structs (via derive macros)
- built-in XML-escaping and un-escaping of string arguments
- built-in date & time parsing for the `dateTime.iso8861` value type
- built-in base64 en- and decoding of byte vectors for the `base64` type
- optional support for (non-standard) `<i8>` (64-bit unsigned integer) and `<nil/>` values
- support for arbitrary method call argument types without needing to convert values
  first (for up to 8 arguments; support for more could be implemented, if needed)
- basic support for both XML-RPC clients (with `reqwest`) and servers (with `axum`)

All conversion methods (both between Rust XML-RPC values and XML strings, and between
Rust primitives and Rust XML-RPC values) are extensively checked for correctness by unit
tests and property-based tests using `quickcheck`.

## Limitations

The implementation of XML-RPC provided by `dxr` also has a few limitations (which might or
might not be deal-breakers for specific use cases):

- Only valid UTF-8 is currently supported in both XML-RPC requests and responses. This is
  a limitation of the `serde` support of `quick-xml`. Support for other encodings could
  be added by implementing custom clients or servers which handle other encodings
  transparently.
- All `dateTime.iso8861` values are assumed to be UTC, as the `dateTime.iso8861` type of
  XML-RPC does not include a timezone. Clients will need to adjust these values according
  to the server timezone. 
- The default client implementation (based on `reqwest`) is currently `async`-only.
  However, adding a "blocking" client implementation based on `reqwest::blocking` should be
  relatively straightforward.
- The default server implementation (based on `axum`) and associated traits are `async`-only.

## Components

- `dxr`: implementation of XML-RPC types, conversion traits between XML-RPC types and
  Rust types, and (de)serialization implementations for converting between XML strings and
  XML-RPC values
- `dxr_derive`: `TryFromDXR` and `TryToDxr` derive macros for custom data types
- `dxr_client`: XML-RPC client implementation using `reqwest`
- `dxr_server`: generic XML-RPC server functionality
- `dxr_server_axum`: XML-RPC server implementation using `axum`

## Why another crate for XML-RPC?

Searching for `xml-rpc` on crates.io yields a few results, but they all did not fit my
use case, or were very hard to use. Either they didn't support implementing both clients
and servers, or no easy conversion methods from Rust types to XML-RPC types was available.
And none of the crates supports (de)serializing both Rust types *and* custom user-defined
types by using derive macros.

## Goals

Because of this state of the XML-RPC crate ecosystem in Rust, the defining purpose of the
`dxr` crate is that it should be opinionated, but also very easy to use, for implementing
both XML-RPC clients and servers, with first-class support for (de)serializing custom
types, in addition to built-in support for transparently converting Rust primitives to
XML-RPC values.

Additionally, the crate is built on top of best-in-class (in my opinion) libraries for
(de)serializing XML (`quick-xml`), HTTP client side (`reqwest`), HTTP server side
(`axum`).
