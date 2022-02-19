use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use axum::http::HeaderMap;
use axum::routing::post;
use axum::Router;

use crate::server::handler::Handler;
use crate::server::{server, DEFAULT_SERVER_ROUTE};

const DEFAULT_SLEEP: Duration = Duration::from_secs(5);

/// builder that takes parameters for constructing a standalone [`axum::Router`]
#[cfg_attr(docsrs, doc(cfg(feature = "axum-server")))]
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
            post(move |body: String, headers: HeaderMap| async move { server(handlers, &body, &headers) }),
        )
    }
}

/// # XML-RPC server implementation
///
/// This type provides a very simple XML-RPC server implementation based on [`axum::Router`].
#[cfg_attr(docsrs, doc(cfg(feature = "axum-server")))]
pub struct Server {
    addr: SocketAddr,
    route: Router,
    off_switch: Option<Box<dyn ServerOffSwitch>>,
}

impl Debug for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Server")
            .field("addr", &self.addr)
            .field("route", &self.route)
            .field("off_switch", &self.off_switch)
            .finish()
    }
}

impl Server {
    /// This method can be used to construct a [`Server`] from a standalone [`axum::Router`], which
    /// will only handle requests at that one route.
    pub fn from_route(addr: SocketAddr, route: Router) -> Server {
        Server {
            addr,
            route,
            off_switch: None,
        }
    }

    /// This method can be used to add a [`ServerOffSwitch`] for handling graceful server shutdown.
    pub fn with_off_switch(mut self, switch: Box<dyn ServerOffSwitch>) -> Self {
        self.off_switch = Some(switch);
        self
    }

    /// This method launches an [`axum::Server`] with the configured route of the XML-RPC endpoint
    /// as the only route that will accept requests.
    ///
    /// Requests with invalid input, calls of unknown methods, and failed methods are converted
    /// into fault responses.
    pub async fn serve(self) -> Result<(), anyhow::Error> {
        if let Some(switch) = &self.off_switch {
            Ok(axum::Server::bind(&self.addr)
                .serve(self.route.into_make_service())
                .with_graceful_shutdown(switch.watch())
                .await?)
        } else {
            Ok(axum::Server::bind(&self.addr)
                .serve(self.route.into_make_service())
                .await?)
        }
    }
}

/// trait definition for server off switches that can be used to handle graceful shutdown
#[async_trait::async_trait]
#[cfg_attr(docsrs, doc(cfg(feature = "axum-server")))]
pub trait ServerOffSwitch: Debug + Send + Sync {
    /// method for checking the state of the off switch
    fn state(&self) -> bool;

    /// method for flipping the off switch
    fn flip(&self);

    /// async method that sleeps for an arbitrary amount of time
    ///
    /// This determines how often the state of the off switch is checked.
    async fn sleep(&self);

    /// async method that checks the state of the server off switch
    ///
    /// This method only ever yields its future value once the state of the switch is flipped.
    async fn watch(&self) {
        loop {
            if self.state() {
                return;
            } else {
                self.sleep().await;
            }
        }
    }
}

/// implementation of [`ServerOffSwitch`] based on tokio
#[derive(Clone, Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "axum-server")))]
pub struct TokioOffSwitch {
    state: Arc<RwLock<bool>>,
    sleep: Duration,
}

impl Default for TokioOffSwitch {
    fn default() -> Self {
        TokioOffSwitch::new()
    }
}

impl TokioOffSwitch {
    /// constructor for [`TokioOffSwitch`] with default settings (sleeping 5 seconds between
    /// checks of the "off switch state")
    pub fn new() -> TokioOffSwitch {
        TokioOffSwitch {
            state: Arc::new(RwLock::new(false)),
            sleep: DEFAULT_SLEEP,
        }
    }

    /// This method makes it possible to override the default sleep duration between checks of
    /// the "off switch state". Short durations will result in faster responses to server
    /// shutdown requests, but will have a higher performance impact.
    pub fn set_sleep_duration(&mut self, duration: Duration) {
        self.sleep = duration;
    }
}

#[async_trait::async_trait]
impl ServerOffSwitch for TokioOffSwitch {
    fn state(&self) -> bool {
        *self.state.read().expect("Poisoned lock!")
    }

    fn flip(&self) {
        let mut state = self.state.write().expect("Poisoned lock!");
        *state = true;
    }

    async fn sleep(&self) {
        tokio::time::sleep(self.sleep).await
    }
}
