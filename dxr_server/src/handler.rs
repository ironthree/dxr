use http::HeaderMap;

use crate::{Fault, Value};

/// type alias for the result type of method handlers
pub type HandlerResult = Result<Option<Value>, Fault>;

/// trait describing server methods that can be called via XML-RPC
///
/// Handlers for XML-RPC method calls must implement this trait. It is already implemented for `fn`
/// functions with the same arguments as the `handler` method in this trait.
///
/// For method handlers that need to keep track of some state (or handle authentication, etc.), just
/// implement this trait for your own struct.
pub trait Handler: Send + Sync {
    /// This method is called for handling incoming XML-RPC method requests with the method name
    /// registered for this [`Handler`], with the request's method parameters as its arguments.
    fn handle(&self, params: &[Value], headers: &HeaderMap) -> HandlerResult;
}

/// type alias for plain handler functions without associated data
pub type HandlerFn = fn(params: &[Value], headers: &HeaderMap) -> HandlerResult;

impl Handler for HandlerFn {
    fn handle(&self, params: &[Value], headers: &HeaderMap) -> HandlerResult {
        self(params, headers)
    }
}
