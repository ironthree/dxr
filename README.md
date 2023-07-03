# dxr: Declarative XML-RPC

The `dxr` crate provides types, macros, and other functionality which can be used to write
fast and correct XML-RPC clients and servers in Rust conveniently.

The APIs for implementing both clients (in the `dxr_client` crate) and servers (in the
`dxr_server` crate) are designed to require no boilerplate code, and implement type
conversions from Rust to XML-RPC types automatically for all supported data types. Custom struct
types are also supported, if they derive or manually implement the `TryFromValue` and / or
`TryToValue` traits from the `dxr` crate.

## Client interface

A new XML-RPC client is initialized by creating a `dxr_client::ClientBuilder` instance for a
specific XML-RPC server URL, modifying it with custom settings, and then building it into a
`dxr_client::Client`. This requires one client backend to be enabled (currently, only
`reqwest` is supported).

```rust
use dxr_client::{Client, ClientBuilder, Url};

let url = Url::parse("https://example.com/xml-rpc/").unwrap();
let client: Client = ClientBuilder::new(url)
    .user_agent("dxr-client-example")
    .build();
```

This client can then be used to issue Remote Procedure `dxr_client::Call`s:

```rust
use dxr_client::Call;

// create an RPC request with one string argument and an expected string return value
let request = Call::new("hello", "DXR");
let result: String = client.call(request).await.unwrap();
```

The `dxr_tests/examples/client.rs` file contains a complete implementation of a simple
"client" binary, which can be used to issue an RPC request to the server provided by the
"server" example.

## Server interface

The APIs for setting up an XML-RPC server are intended to be similarly straight-forward,
and allow embedding the XML-RPC server endpoint route into other servers. First, set up a
`dxr_server::RouteBuilder`, set up all method handlers, build it into an
`dxr_server::axum::Router`, and then either use this route as part of a larger server,
or create a standalone service from it. This requires one server backend to be enabled
(currently, only `axum` is supported).


```rust
use dxr_server::RouteBuilder;
let route = RouteBuilder::new().build();
```

Now, this is not a very useful XML-RPC endpoint, since it does not know about any method calls.
An arbitrary number of method handlers can be registered with the [`dxr_server::RouteBuilder`]
before building the [`dxr_server::axum::Router`].

```rust
use dxr::{Fault, TryFromParams, TryToValue, Value};
use dxr_server::{HandlerFn, HandlerResult};
use dxr_server::{axum::http::HeaderMap, RouteBuilder};

fn hello_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    let name = String::try_from_params(params)?;
    Ok(format!("Handler function says: Hello, {}!", name).try_to_value()?)
}

let route = RouteBuilder::new()
    .set_path("/")
    .add_method("hello", Box::new(hello_handler as HandlerFn))
    .build();
```

Method handlers must either implement `dxr_server::Handler` themselves, or align with the
`dxr_server::HandlerFn` function pointer type, for which this trait implementation is
already provided.

Using this route in a standalone server with only an XML-RPC endpoint is straightforward:

```rust
use dxr_server::Server;

let server = Server::from_route(route);
server.serve("0.0.0.0:3000".parse().unwrap()).await.unwrap();
```

The `dxr_tests/examples/server.rs` file contains an implementation of a simple server binary, which
provides a `hello(String)` method that returns a welcome message, and a `countme()` method that
returns the number of times the `countme()` method has been called since the server was started.

## Optional Features

The `dxr` crate provides functionality for deriving the `TryFromDXR` and `TryToDXR` traits
if the `derive` feature is enabled.

There is also optional support for two common, non-standard XML-RPC extensions:

- "long" 64-bit integers (`<i8>`): mapped to [`i64`], enabled with the `i8` feature
- "null" values (`<nil/>`): mapped to [`Option`]`<T>`, enabled with the `nil` feature
