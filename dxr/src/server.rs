use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use dxr_shared::{Fault, FaultResponse, MethodCall, MethodResponse, Value};

use axum::routing::post;
use axum::Router;

pub trait Handler: Send + Sync {
    fn handle(&self, params: &Vec<Value>) -> Result<Value, Fault>;
}

pub struct ServerBuilder {
    addr: SocketAddr,
    handlers: HashMap<&'static str, Box<dyn Handler>>,
}

impl ServerBuilder {
    pub fn new(addr: SocketAddr) -> ServerBuilder {
        ServerBuilder {
            addr,
            handlers: HashMap::new(),
        }
    }

    pub fn add_method(mut self, name: &'static str, handler: Box<dyn Handler>) -> Self {
        self.handlers.insert(name, handler);
        self
    }

    pub fn build(self) -> Server {
        Server {
            addr: self.addr,
            handlers: Arc::new(self.handlers),
        }
    }
}

pub struct Server {
    addr: SocketAddr,
    handlers: Arc<HashMap<&'static str, Box<dyn Handler>>>,
}

impl Server {
    pub async fn serve(self) {
        let app = Router::new().route(
            "/",
            post({
                move |body: String| async move {
                    let call: MethodCall = match quick_xml::de::from_str(&body) {
                        Ok(call) => call,
                        Err(error) => return fault_to_body(-1, &format!("Invalid request input: {}", error)),
                    };

                    let handler = self.handlers.get(call.name());

                    let result = match handler {
                        Some(handler) => handler.handle(call.params()),
                        None => return fault_to_body(-1000, "Unknown method."),
                    };

                    let response = match result {
                        Ok(value) => response_to_body(value),
                        Err(fault) => fault_to_body(fault.code(), fault.string()),
                    };

                    response
                }
            }),
        );

        axum::Server::bind(&self.addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

fn response_to_body(value: Value) -> String {
    let response = MethodResponse::new(value);

    quick_xml::se::to_string(&response).unwrap()
}

fn fault_to_body(code: i32, string: &str) -> String {
    let fault = Fault::new(code, string.to_owned());
    let response: FaultResponse = fault.into();

    quick_xml::se::to_string(&response).unwrap()
}
