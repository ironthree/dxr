//! # Simple example client
//!
//! Run this example to test interaction with the `server` example.

use dxr::url::Url;
use dxr::{Call, ClientBuilder};

#[tokio::main]
async fn main() -> Result<(), String> {
    let url = Url::parse("http://0.0.0.0:3000/").expect("Failed to parse hardcoded URL.");

    let client = ClientBuilder::new(url).user_agent("dxr-client-example").build();

    let request: Call<_, String> = Call::new("hello", "DXR");
    let result = client.call(request).await.map_err(|error| error.to_string())?;

    // print query result
    println!("{}", result);

    Ok(())
}
