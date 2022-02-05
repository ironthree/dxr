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
