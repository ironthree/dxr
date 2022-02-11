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
