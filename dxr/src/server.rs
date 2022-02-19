use std::collections::HashMap;
use std::sync::Arc;

use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use http::{HeaderMap, HeaderValue, StatusCode};

use crate::error::DxrError;
use crate::fault::Fault;
use crate::types::{FaultResponse, MethodCall, MethodResponse, Value};

mod handler;
pub use handler::*;

mod support;
pub use support::*;

/// default server route / path for XML-RPC endpoints
#[cfg_attr(docsrs, doc(cfg(feature = "server")))]
pub const DEFAULT_SERVER_ROUTE: &str = "/";

/// type alias for atomically reference-counted map of XML-RPC method names and handlers
#[cfg_attr(docsrs, doc(cfg(feature = "server")))]
pub type HandlerMap = Arc<HashMap<&'static str, Box<dyn Handler>>>;

/// This function can be used in custom XML-RPC endpoints (BYOS - bring your own server).
///
/// It takes a map of method handlers ([`HandlerMap`]), the request body, and the request headers
/// as arguments, and returns a tuple of HTTP status code [`http::StatusCode`], request
/// response headers, and response body.
#[cfg_attr(docsrs, doc(cfg(feature = "server")))]
pub fn server(handlers: HandlerMap, body: &str, headers: &HeaderMap) -> (StatusCode, HeaderMap, String) {
    if headers.get(CONTENT_LENGTH).is_none() {
        return fault_to_response(411, "Content-Length header missing.");
    }

    let call: MethodCall = match quick_xml::de::from_str(body) {
        Ok(call) => call,
        Err(error) => {
            let e = DxrError::invalid_data(error.to_string());
            let f = Fault::new(400, e.to_string());
            return fault_to_response(f.code(), f.string());
        },
    };

    let handler = match handlers.get(call.name()) {
        Some(handler) => handler,
        None => return fault_to_response(404, "Unknown method."),
    };

    let response = match handler.handle(call.params(), headers) {
        Ok(value) => success_to_response(value),
        Err(fault) => fault_to_response(fault.code(), fault.string()),
    };

    response
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
