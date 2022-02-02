use axum::http::StatusCode;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::net::SocketAddr;
use std::sync::Arc;

use dxr_shared::{Fault, FaultResponse, MethodCall, MethodResponse, Value};

use axum::routing::post;
use axum::Router;

/// trait describing server methods that can be called via XML-RPC
pub trait Handler: Send + Sync {
    /// This method is called for handling incoming XML-RPC method requests with the method name
    /// registered for this [`Handler`], with the request's method parameters as its arguments.
    fn handle(&self, params: &[Value]) -> Result<Value, Fault>;
}

/// builder that takes parameters for constructing a [`Server`]
pub struct ServerBuilder {
    addr: SocketAddr,
    handlers: HashMap<&'static str, Box<dyn Handler>>,
}

impl Debug for ServerBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut handler_list: Vec<&&str> = self.handlers.keys().collect();
        handler_list.sort();

        f.debug_struct("ServerBuilder")
            .field("addr", &self.addr)
            .field("handlers", &handler_list)
            .finish()
    }
}

impl ServerBuilder {
    /// constructor for [`ServerBuilder`] from the address of the XML-RPC server
    pub fn new(addr: SocketAddr) -> ServerBuilder {
        ServerBuilder {
            addr,
            handlers: HashMap::new(),
        }
    }

    /// method for adding a new method handler for the [`Server`]
    pub fn add_method(mut self, name: &'static str, handler: Box<dyn Handler>) -> Self {
        self.handlers.insert(name, handler);
        self
    }

    /// build the [`Server`] from the specified URL and registered method handlers
    pub fn build(self) -> Server {
        Server {
            addr: self.addr,
            handlers: Arc::new(self.handlers),
        }
    }
}

/// # XML-RPC server implementation
///
/// This type provides a very simple XML-RPC server implementation. Specify server address,
/// register method handlers, initialize the [`Server`], and wait for requests.
pub struct Server {
    addr: SocketAddr,
    handlers: Arc<HashMap<&'static str, Box<dyn Handler>>>,
}

impl Debug for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut handler_list: Vec<&&str> = self.handlers.keys().collect();
        handler_list.sort();

        f.debug_struct("ServerBuilder")
            .field("addr", &self.addr)
            .field("handlers", &handler_list)
            .finish()
    }
}

impl Server {
    /// asynchronous method for processing remote procedure calls via XML-RPC
    ///
    /// Requests with invalid input, calls of unknown methods, and failed methods are converted
    /// into fault responses.
    pub async fn serve(self) {
        let app = Router::new().route(
            "/",
            post({
                move |body: String| async move {
                    let call: MethodCall = match quick_xml::de::from_str(&body) {
                        Ok(call) => call,
                        Err(error) => return fault_to_response(-1, &format!("Invalid request input: {}", error)),
                    };

                    let handler = self.handlers.get(call.name());

                    let result = match handler {
                        Some(handler) => handler.handle(call.params()),
                        None => return fault_to_response(-1000, "Unknown method."),
                    };

                    let response = match result {
                        Ok(value) => success_to_response(value),
                        Err(fault) => fault_to_response(fault.code(), fault.string()),
                    };

                    response
                }
            }),
        );

        axum::Server::bind(&self.addr)
            .serve(app.into_make_service())
            .await
            .expect("Failed to initialize server.");
    }
}

fn success_to_response(value: Value) -> (StatusCode, String) {
    let response = MethodResponse::new(value);

    match quick_xml::se::to_string(&response) {
        Ok(success) => (StatusCode::OK, success),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
    }
}

fn fault_to_response(code: i32, string: &str) -> (StatusCode, String) {
    let fault = Fault::new(code, string.to_owned());
    let response: FaultResponse = fault.into();

    match quick_xml::se::to_string(&response) {
        Ok(fault) => (StatusCode::OK, fault),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
    }
}
