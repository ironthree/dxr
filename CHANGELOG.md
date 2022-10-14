## Unreleased (0.5.0)

**Changed**:

All public traits were renamed to match Rust conventions for conversions ("try" prefix for
fallible conversions, "to" for conversions that don't take ownership):

- `FromParams` was renamed to `TryFromParams`
- `FromDXR` was renamed to `TryFromValue`
- `ToParams` was renamed to `TryToParams`
- `ToDXR` was renamed to `TryToValue`

## Release 0.4.0

**Changed**:

- moved implementations of XML-RPC clients, servers, and the axum server support into separate
  crates, available as optional features of the top-level `dxr` crate
- moved to more powerful cargo feature syntax for optional and conditional dependencies
- server: refactored method call `Handler` trait to allow the handler function to be `async`

**Fixed**:

- `methodResponse` must apparently always contain exactly one value
- `methodParameters` contain an array of parameters that contain values, not a parameter that
  is an array of values

Both these fixes required minor changes to the public API, which is why they cannot be backported
to the 0.3.x branch.

**Added**:

- implemented of `FromDXR` for fixed-size arrays
- implemented support for struct fields that are fixed-size arrays in the derive macros

**Updated dependencies**:

- updated `axum` version from 0.4 to 0.5
- updated `quick_xml` version from 0.22 to 0.23

## Release 0.3.1

Fixed:

- fixed re-export of `async-trait` if `axum-server` feature is not enabled 😫

## Release 0.3.0

Added:

- implementations of `ToDXR` and `FromDXR` for `Box<T>` (making owned recursive types possible)
- support for deriving `ToDXR` for structs containing `&T` references as fields (making
  reference-based recursive types possible)

Changed:

- merged code from the `dxr_shared` crate into the main `dxr` crate
- removed support for non-standard `i8` and `nil` values from default features
- split server feature into generic server functionality and `axum` support
- use `anyhow` to simplify error handling in client and server implementations
- conversion from `FaultResponse` to `Fault` can fail, so implement a fallible
  `TryFrom` instead of a panicking `From`

Internal changes:

- added `trybuild` based pass/fail tests for derive macros

## Release 0.2.1

Added:

- `ServerBuilder` method for changing the default route of the XML-RPC endpoint (default: `/`)

Internal changes:

- simplified / refactored `Server::serve` by splitting off the XML-RPC endpoint handler

## Release 0.2.0

This version contains some fixes that required changes in public APIs (listed below), so this
is a semver-incompatible release.

Added:

- convenience methods for checking the type of `DxrError` and extracting the inner error value
- more client-server roundtrip tests (for ~80% test coverage across all three crates)
- support for implementing / requesting graceful server shutdown
- snapshot tests for some `Debug` implementations
- feature hints for the generated documentation on [docs.rs](https://docs.rs/dxr)

Changed:

- moved method handler synchronization from the server to the method handlers themselves to
  improve concurrency, throughput and latency, especially for methods that do not modify internal
  or global state
- changed `MethodResponse` to match the XML-RPC spec (`<params>` can be missing)
- changed method signatures on client and server implementations to adapt to optional return values
- renamed `DxrError::ReturnMismatch` to `DxrError::ParameterMismatch` to better reflect what it
  actually means (unexpected number of parameters, whether they are method call arguments or
  return values)

## Release 0.1.1

This version fixes some minor issues that were discovered since the last release.

Added:

- added some client-server roundtrip tests using a simple echo server implementation
  (in `dxr/tests/echo.rs`)

Fixed:

- fixed infinite recursion in `(T,)` call parameter serialization 
- make sure `Call` values can be sent across threads
- cleaned up minor formatting issues in docs and doctests
- fixed README.md badge links in the dxr_shared crate

## Release 0.1.0

Initial release.
