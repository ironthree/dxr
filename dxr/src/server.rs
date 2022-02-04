use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::net::SocketAddr;
use std::sync::Arc;

use dxr_shared::{Fault, FaultResponse, MethodCall, MethodResponse, Value};

use axum::http::{header::CONTENT_LENGTH, header::CONTENT_TYPE, HeaderMap, HeaderValue, StatusCode};
use axum::routing::post;
use axum::Router;

/// trait describing server methods that can be called via XML-RPC
pub trait Handler: Send + Sync {
    /// This method is called for handling incoming XML-RPC method requests with the method name
    /// registered for this [`Handler`], with the request's method parameters as its arguments.
    fn handle(&self, params: &[Value], headers: &HeaderMap) -> Result<Value, Fault>;
}

/// type alias for plain handler functions without associated data
pub type HandlerFn = fn(params: &[Value], headers: &HeaderMap) -> Result<Value, Fault>;

impl Handler for HandlerFn {
    fn handle(&self, params: &[Value], headers: &HeaderMap) -> Result<Value, Fault> {
        self(params, headers)
    }
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
    pub async fn serve(self) -> Result<(), String> {
        let app = Router::new().route(
            "/",
            post({
                move |body: String, headers: HeaderMap| async move {
                    if headers.get(CONTENT_LENGTH).is_none() {
                        return fault_to_response(411, "Content-Length header missing.");
                    }

                    let call: MethodCall = match quick_xml::de::from_str(&body) {
                        Ok(call) => call,
                        Err(error) => return fault_to_response(400, &format!("Invalid request input: {}", error)),
                    };

                    let handler = match self.handlers.get(call.name()) {
                        Some(handler) => handler,
                        None => return fault_to_response(404, "Unknown method."),
                    };

                    let response = match handler.handle(call.params(), &headers) {
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
            .map_err(|error| error.to_string())
    }
}

fn response_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/xml"));
    headers
}

fn success_to_response(value: Value) -> (StatusCode, HeaderMap, String) {
    let response = MethodResponse::new(value);

    match quick_xml::se::to_string(&response) {
        Ok(success) => (StatusCode::OK, response_headers(), success),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, response_headers(), error.to_string()),
    }
}

fn fault_to_response(code: i32, string: &str) -> (StatusCode, HeaderMap, String) {
    let fault = Fault::new(code, string.to_owned());
    let response: FaultResponse = fault.into();

    match quick_xml::se::to_string(&response) {
        Ok(fault) => (StatusCode::OK, response_headers(), fault),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, response_headers(), error.to_string()),
    }
}
