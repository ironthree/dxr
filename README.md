# DXR: declarative XML-RPC

This repository contains work-in-progress crates for writing XML-RPC API clients and
servers in Rust.

The following features are already implemented:

- (de)serialization support for converting XML-RPC XML strings into strongly-typed Rust values
- conversion traits between XML-RPC values and Rust primitives, arrays, slices, byte arrays,
  tuples, hashmaps, and custom structs (via derive macros)
- built-in XML-escaping and un-escaping of string arguments
- built-in date & time parsing for the `dateTime.iso8861` value type
- built-in base64 en- and decoding of byte vectors for the `base64` type
- optional support for the (non-standard) `<i8>` (64-bit unsigned integer) and `<nil/>`
  (enabled by default)
- support for arbitrary method call argument types without needing to convert values
  first (for up to 8 arguments; support for more could be implemented, if needed)
- basic support for both XML-RPC clients (with `reqwest`) and servers (with `axum`)

All conversion methods (both between Rust XML-RPC values and XML strings, and between
Rust primitives and Rust XML-RPC values) are extensively checked for correctness by unit
tests and property-based tests using `quickcheck`.

The project is split into three crates:

- `dxr`: high-level method call, client, and server implementations
- `dxr_derive`: `ToDXR` and `FromDXR` derive macros for custom structs
- `dxr_shared`: definition of conversion traits and XML-RPC types for (de)serialization  

All relevant parts of `dxr_derive` and `dxr_shared` are re-exported in the `dxr` crate,
they are not supposed to be used directly.

### Why another crate for XML-RPC?

Searching for `xml-rpc` on crates.io yields a few results, but they all did not fit my
use case, or were very hard to use. Either they didn't support implementing both clients
and servers, or no easy conversion methods from Rust types to XML-RPC types was available.
And none of the crates supports (de)serializing both Rust types *and* custom user-defined
types by using derive macros.

### Goals

Because of this state of the XML-RPC crate ecosystem in Rust, the defining purpose of the
`dxr` crate is that it should be opinionated, but also very easy to use, for implementing
both XML-RPC clients and servers, with first-class support for (de)serializing custom
types, in addition to built-in support for transparently converting Rust primitives to
XML-RPC values.

Additionally, the crate is built on top of best-in-class (in my opinion) libraries for
(de)serializing XML (`quick-xml`), HTTP client side (`reqwest`), HTTP server side
(`axum`).

### Examples

The `/examples/` directory contains implementations of two simple clients and a simple
server for demonstration purposes. They use the `tokio` runtime, which works great with
both `reqwest` and `axum`.

Note that the amount of code that is required for writing simple XML-RPC clients and
servers is very small. The `client` example has only ~10 LOC, and the `server` example
only needs ~25 LOC, but both examples even include error handling.
