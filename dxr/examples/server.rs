//! # Simple example server
//!
//! Run this example with `cargo run --example server --features server`. It will listen on
//! <http://0.0.0.0:3000> for incoming XML-RPC requests.
//!
//! Testing this local server is straightforward, either with the included `client` example, or
//! with three lines of python:
//!
//! ```python3
//! >>> import xmlrpc.client
//! >>> proxy = xmlrpc.client.ServerProxy("http://0.0.0.0:3000/")
//! >>> proxy.hello("DXR")
//! 'Hello, DXR!'
//! ```

use axum::http::HeaderMap;
use dxr::{DxrError, Fault, FromDXR, Handler, ServerBuilder, ToDXR, Value};

struct HelloHandler {}

impl Handler for HelloHandler {
    fn handle(&self, params: &[Value], _headers: &HeaderMap) -> Result<Value, Fault> {
        let mut params = params
            .iter()
            .map(FromDXR::from_dxr)
            .collect::<Result<Vec<String>, DxrError>>()
            .map_err(|error| Fault::new(500, error.to_string()))?;

        let name = match params.len() {
            1 => params.remove(0),
            n => return Err(Fault::new(400, format!("Expected one argument, got {}.", n))),
        };

        format!("Hello, {}!", name)
            .to_dxr()
            .map_err(|error| Fault::new(500, error.to_string()))
    }
}

#[tokio::main]
async fn main() {
    let hello_handler = HelloHandler {};

    let server = ServerBuilder::new("0.0.0.0:3000".parse().unwrap())
        .add_method("hello", Box::new(hello_handler))
        .build();

    server.serve().await.expect("Failed to run server.")
}
