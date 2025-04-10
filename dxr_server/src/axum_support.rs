#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![deny(unsafe_code)]
#![warn(explicit_outlives_requirements)]
#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(noop_method_call)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unreachable_pub)]
#![warn(clippy::unwrap_used)]

//! # dxr_server_axum
//!
//! This crate provides an implementation of an XML-RPC server based on [`axum`] built on top of
//! [`dxr`] and [`dxr_server`].

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::HeaderMap;
use axum::routing::post;
use axum::serve::Listener;
use axum::Router;

use thiserror::Error;
use tokio::net::TcpListener;
use tokio::sync::Notify;

use crate::{server, Handler, DEFAULT_SERVER_ROUTE};

/// error type for XML-RPC servers
#[derive(Debug, Error)]
pub enum ServerError {
    /// error variant for HTTP server errors
    #[error("{}", error)]
    Http {
        /// HTTP server error
        #[from]
        error: hyper::Error,
    },
    /// error variant for networking errors
    #[error("{}", error)]
    Net {
        /// error returned by [`TcpListener::bind`]
        #[from]
        error: std::io::Error,
    },
}

/// builder that takes parameters for constructing a standalone [`axum::Router`]
#[derive(Default)]
pub struct RouteBuilder {
    path: Cow<'static, str>,
    handlers: HashMap<&'static str, Box<dyn Handler>>,
}

impl Debug for RouteBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut handler_list: Vec<&&str> = self.handlers.keys().collect();
        handler_list.sort();

        f.debug_struct("ServerBuilder")
            .field("path", &self.path)
            .field("handlers", &handler_list)
            .finish()
    }
}

impl RouteBuilder {
    /// constructor for [`RouteBuilder`] from the address of the XML-RPC server
    pub fn new() -> RouteBuilder {
        RouteBuilder {
            path: Cow::Borrowed(DEFAULT_SERVER_ROUTE),
            handlers: HashMap::new(),
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

    /// method for adding a new method handler
    pub fn add_method(mut self, name: &'static str, handler: Box<dyn Handler>) -> Self {
        self.handlers.insert(name, handler);
        self
    }

    /// build an [`axum::Router`] from the specified route and registered method handlers
    pub fn build(self) -> Router {
        let handlers = Arc::new(self.handlers);
        Router::new().route(
            self.path.as_ref(),
            post(move |headers: HeaderMap, body: String| async move { server(handlers, &body, headers).await }),
        )
    }
}

/// # XML-RPC server implementation
///
/// This type provides a very simple XML-RPC server implementation based on [`axum::Router`].
#[derive(Debug)]
pub struct Server {
    route: Router,
    barrier: Option<Arc<Notify>>,
}

impl Server {
    /// This method can be used to construct a [`Server`] from a standalone [`axum::Router`], which
    /// will only handle requests at that one route.
    pub fn from_route(route: Router) -> Server {
        Server { route, barrier: None }
    }

    /// This method adds a barrier / notifier to the server that will trigger graceful shutdown,
    /// and returns a reference to it, which can be used to trigger graceful server shutdown upon
    /// request.
    pub fn shutdown_trigger(&mut self) -> Arc<Notify> {
        let barrier = Arc::new(Notify::new());
        self.barrier = Some(barrier.clone());
        barrier
    }

    /// This method launches a server that listens at the specified socket address with the
    /// configured route of the XML-RPC endpoint as the only route that will accept requests.
    ///
    /// Requests with invalid input, calls of unknown methods, and failed methods are converted
    /// into fault responses.
    pub async fn serve(self, addr: SocketAddr) -> Result<(), ServerError> {
        let listener = TcpListener::bind(addr).await?;
        self.serve_listener(listener).await
    }

    /// This method launches a server  that uses the supplied [`TcpListener`] and with the
    /// configured route of the XML-RPC endpoint as the only route that will accept requests.
    ///
    /// Requests with invalid input, calls of unknown methods, and failed methods are converted
    /// into fault responses.
    pub async fn serve_listener<L>(self, listener: L) -> Result<(), ServerError>
    where
        L: Listener,
        L::Addr: Debug,
    {
        if let Some(barrier) = self.barrier {
            Ok(axum::serve(listener, self.route)
                .with_graceful_shutdown(async move { barrier.notified().await })
                .await?)
        } else {
            Ok(axum::serve(listener, self.route).await?)
        }
    }
}
