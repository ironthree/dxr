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
//! >>> proxy.add(1, 2)
//! 3
//! ```

use std::sync::RwLock;

use dxr::server::{async_trait, Handler, HandlerFn, HandlerResult};
use dxr::server_axum::{axum::http::HeaderMap, RouteBuilder, Server};
use dxr::{TryFromParams, TryToValue, Value};

struct CounterHandler {
    counter: RwLock<u32>,
}

impl CounterHandler {
    fn new(init: u32) -> CounterHandler {
        CounterHandler {
            counter: RwLock::new(init),
        }
    }
}

#[async_trait]
impl Handler for CounterHandler {
    async fn handle(&self, _params: &[Value], _headers: HeaderMap) -> HandlerResult {
        let mut value = self.counter.write().unwrap();
        let result = (*value as i32).try_to_value()?;
        *value += 1;
        Ok(result)
    }
}

fn hello_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    let name = String::try_from_params(params)?;
    Ok(format!("Handler function says: Hello, {}!", name).try_to_value()?)
}

fn adder_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    let (a, b): (i32, i32) = TryFromParams::try_from_params(params)?;
    Ok((a + b).try_to_value()?)
}

#[tokio::main]
async fn main() {
    let counter_handler = CounterHandler::new(0);

    let route = RouteBuilder::new()
        .set_path("/")
        .add_method("hello", Box::new(hello_handler as HandlerFn))
        .add_method("countme", Box::new(counter_handler))
        .add_method("add", Box::new(adder_handler as HandlerFn))
        .build();

    let server = Server::from_route("0.0.0.0:3000".parse().unwrap(), route);

    server.serve().await.expect("Failed to run server.")
}
