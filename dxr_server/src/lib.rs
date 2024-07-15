#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

//! # dxr_server
//!
//! This crate provides generic XML-RPC server functionality based on [`dxr`].

use std::collections::HashMap;
use std::sync::Arc;

use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use http::{HeaderMap, HeaderValue, StatusCode};

use dxr::{DxrError, Fault, FaultResponse, MethodCall, MethodResponse, Value};

mod handler;
pub use handler::*;

#[cfg(feature = "axum")]
mod axum_support;
#[cfg(feature = "axum")]
pub use self::axum_support::*;

// re-export axum, as it is exposed in the the public API
#[cfg(feature = "axum")]
pub use axum;

// re-export the async_trait macro, as it is exposed as part of the public API
pub use async_trait::async_trait;

/// default server route / path for XML-RPC endpoints
pub const DEFAULT_SERVER_ROUTE: &str = "/";

/// type alias for atomically reference-counted map of XML-RPC method names and handlers
pub type HandlerMap = Arc<HashMap<&'static str, Box<dyn Handler>>>;

/// This function can be used in custom XML-RPC endpoints (BYOS - bring your own server).
///
/// It takes a map of method handlers ([`HandlerMap`]), the request body, and the request headers
/// as arguments, and returns a tuple of HTTP status code [`http::StatusCode`], request
/// response headers, and response body.
pub async fn server(handlers: HandlerMap, body: &str, headers: HeaderMap) -> (StatusCode, HeaderMap, String) {
    if headers.get(CONTENT_LENGTH).is_none() {
        return fault_to_response(411, "Content-Length header missing.");
    }

    let call: MethodCall = match dxr::deserialize_xml(body) {
        Ok(call) => call,
        Err(error) => {
            let e = DxrError::invalid_data(error.to_string());
            let f = Fault::from(e);
            return fault_to_response(f.code(), f.string());
        },
    };

    #[cfg(feature = "multicall")]
    if call.name() == "system.multicall" {
        let calls = match dxr::from_multicall_params(call.params()) {
            Ok(calls) => calls,
            Err(error) => {
                let f = Fault::from(error);
                return fault_to_response(f.code(), f.string());
            },
        };

        let mut results = Vec::new();

        for multi in calls {
            match multi {
                Ok((name, params)) => {
                    let handler = match handlers.get(name.as_str()) {
                        Some(handler) => handler,
                        None => {
                            results.push(Err(Fault::new(404, String::from("Unknown method."))));
                            continue;
                        },
                    };

                    let result = handler.handle(&params, headers.clone()).await;
                    results.push(result);
                },
                Err(error) => {
                    results.push(Err(Fault::from(error)));
                    continue;
                },
            }
        }

        let value = dxr::into_multicall_response(results);

        return success_to_response(value);
    }

    let handler = match handlers.get(call.name()) {
        Some(handler) => handler,
        None => return fault_to_response(404, "Unknown method."),
    };

    let response = match handler.handle(&call.params(), headers).await {
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

fn success_to_response(value: Value) -> (StatusCode, HeaderMap, String) {
    let response = MethodResponse::new(value);

    match dxr::serialize_xml(&response) {
        Ok(success) => (StatusCode::OK, response_headers(), success),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, response_headers(), error.to_string()),
    }
}

fn fault_to_response(code: i32, string: &str) -> (StatusCode, HeaderMap, String) {
    let fault = Fault::new(code, string.to_owned());
    let response: FaultResponse = fault.into();

    match dxr::serialize_xml(&response) {
        Ok(fault) => (StatusCode::OK, response_headers(), fault),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, response_headers(), error.to_string()),
    }
}
