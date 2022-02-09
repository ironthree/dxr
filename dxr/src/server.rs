use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::{header::CONTENT_LENGTH, header::CONTENT_TYPE, HeaderMap, HeaderValue, StatusCode};
use axum::routing::post;
use axum::Router;

use dxr_shared::{DxrError, Fault, FaultResponse, MethodCall, MethodResponse, Value};

mod handler;
pub use handler::*;

mod shutdown;
pub use shutdown::*;

/// default server route / path for XML-RPC endpoints
#[cfg_attr(docsrs, doc(cfg(feature = "server")))]
pub const DEFAULT_SERVER_ROUTE: &str = "/";

/// builder that takes parameters for constructing a [`Server`]
#[cfg_attr(docsrs, doc(cfg(feature = "server")))]
pub struct ServerBuilder {
    addr: SocketAddr,
    path: Cow<'static, str>,
    handlers: HashMap<&'static str, Box<dyn Handler>>,
    off_switch: Option<Box<dyn ServerOffSwitch>>,
}

impl Debug for ServerBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut handler_list: Vec<&&str> = self.handlers.keys().collect();
        handler_list.sort();

        f.debug_struct("ServerBuilder")
            .field("addr", &self.addr)
            .field("path", &self.path)
            .field("handlers", &handler_list)
            .field("off_switch", &self.off_switch)
            .finish()
    }
}

impl ServerBuilder {
    /// constructor for [`ServerBuilder`] from the address of the XML-RPC server
    pub fn new(addr: SocketAddr) -> ServerBuilder {
        ServerBuilder {
            addr,
            path: Cow::Borrowed(DEFAULT_SERVER_ROUTE),
            handlers: HashMap::new(),
            off_switch: None,
        }
    }

    /// method for overriding the default path / route for the XML-RPC endpoint
    ///
    /// The default value is [`/`](DEFAULT_SERVER_ROUTE). Another common value
    /// is `/RPC2`, which can be set with this method, if necessary.
    pub fn set_path(mut self, route: &str) -> Self {
        self.path = Cow::Owned(route.to_owned());
        self
    }

    /// method for adding a switch that is used to handle graceful shutdown
    ///
    /// To avoid a direct dependency on an async runtime, the value implementing the
    /// [`ServerOffSwitch`] trait must provide its own state tracking and sleeping logic.
    pub fn add_off_switch(mut self, off_switch: Box<dyn ServerOffSwitch>) -> Self {
        self.off_switch = Some(off_switch);
        self
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
            path: self.path,
            handlers: Arc::new(self.handlers),
            off_switch: self.off_switch,
        }
    }
}

/// # XML-RPC server implementation
///
/// This type provides a very simple XML-RPC server implementation. Specify server address,
/// register method handlers, initialize the [`Server`], and wait for requests.
#[cfg_attr(docsrs, doc(cfg(feature = "server")))]
pub struct Server {
    addr: SocketAddr,
    path: Cow<'static, str>,
    handlers: Arc<HashMap<&'static str, Box<dyn Handler>>>,
    off_switch: Option<Box<dyn ServerOffSwitch>>,
}

impl Debug for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut handler_list: Vec<&&str> = self.handlers.keys().collect();
        handler_list.sort();

        f.debug_struct("Server")
            .field("addr", &self.addr)
            .field("path", &self.path)
            .field("handlers", &handler_list)
            .field("off_switch", &self.off_switch)
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
            self.path.as_ref(),
            post({
                move |body: String, headers: HeaderMap| async move {
                    if headers.get(CONTENT_LENGTH).is_none() {
                        return fault_to_response(411, "Content-Length header missing.");
                    }

                    let call: MethodCall = match quick_xml::de::from_str(&body) {
                        Ok(call) => call,
                        Err(error) => {
                            let e = DxrError::invalid_data(error.to_string());
                            let f = Fault::new(400, e.to_string());
                            return fault_to_response(f.code(), f.string());
                        },
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

        if let Some(switch) = self.off_switch {
            axum::Server::bind(&self.addr)
                .serve(app.into_make_service())
                .with_graceful_shutdown(switch.watch())
                .await
                .map_err(|error| error.to_string())
        } else {
            axum::Server::bind(&self.addr)
                .serve(app.into_make_service())
                .await
                .map_err(|error| error.to_string())
        }
    }
}

fn response_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/xml"));
    headers
}

fn success_to_response(value: Option<Value>) -> (StatusCode, HeaderMap, String) {
    let response = match value {
        Some(value) => MethodResponse::new(value),
        None => MethodResponse::empty(),
    };

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
