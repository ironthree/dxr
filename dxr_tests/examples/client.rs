//! # Simple example client
//!
//! Run this example to test interaction with the `server` example from the
//! `dxr_server_axum` crate.

use dxr_client::{ClientBuilder, Url};

#[tokio::main]
async fn main() -> Result<(), String> {
    let url = Url::parse("http://0.0.0.0:3000/").expect("Failed to parse hardcoded URL.");

    let client = ClientBuilder::new(url).user_agent("dxr-client-example").build();

    let result: String = client.call("hello", "DXR").await.map_err(|error| error.to_string())?;
    println!("Server message: {result}");

    let result: i32 = client.call("countme", ()).await.map_err(|error| error.to_string())?;
    println!("Server counter: {result}");

    let result: i32 = client.call("add", (1, 2)).await.map_err(|error| error.to_string())?;
    println!("1 + 2 = {result}");

    Ok(())
}
