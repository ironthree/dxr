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
//! 'Handler type says: Hello, DXR!'
//! >>> proxy.countme()
//! 0
//! >>> proxy.countme()
//! 1
//! >>> proxy.countme()
//! 2
//! ```

use dxr::axum::http::HeaderMap;
use dxr::{Fault, FromParams, Handler, HandlerFn, ServerBuilder, ToDXR, Value};

struct CounterHandler {
    counter: u32,
}

impl Handler for CounterHandler {
    fn handle(&mut self, _params: &[Value], _headers: &HeaderMap) -> Result<Option<Value>, Fault> {
        let result = (self.counter as i32).to_dxr()?;
        self.counter += 1;
        Ok(Some(result))
    }
}

fn hello_handler(params: &[Value], _headers: &HeaderMap) -> Result<Option<Value>, Fault> {
    let name = String::from_params(params)?;
    Ok(Some(format!("Handler function says: Hello, {}!", name).to_dxr()?))
}

#[tokio::main]
async fn main() {
    let counter_handler = CounterHandler { counter: 0 };

    let server = ServerBuilder::new("0.0.0.0:3000".parse().unwrap())
        .add_method("hello", Box::new(hello_handler as HandlerFn))
        .add_method("countme", Box::new(counter_handler))
        .build();

    server.serve().await.expect("Failed to run server.")
}
