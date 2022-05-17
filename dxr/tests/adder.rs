//! This file implements a test that launches a simple server that can add integers.

use std::time::Duration;

use dxr::axum::http::HeaderMap;
use dxr::{Call, ClientBuilder, Fault, FromParams, HandlerFn, RouteBuilder, Server, ToDXR, Value};

fn add_handler(params: &[Value], _headers: &HeaderMap) -> Result<Option<Value>, Fault> {
    let (a, b): (i32, i32) = FromParams::from_params(params)?;
    Ok(Some((a + b).to_dxr()?))
}

#[tokio::test]
async fn adder() {
    let route = RouteBuilder::new()
        .set_path("/")
        .add_method("add", Box::new(add_handler as HandlerFn))
        .build();

    let mut server = Server::from_route("0.0.0.0:3000".parse().unwrap(), route);
    let trigger = server.shutdown_trigger();

    let serve = tokio::spawn(server.serve());
    tokio::time::sleep(Duration::from_secs(1)).await;

    let calls = || async {
        let client = ClientBuilder::new("http://0.0.0.0:3000/".parse().unwrap())
            .user_agent("echo-client")
            .build();

        // add something with tuple params
        let (a, b) = (2i32, 3i32);
        let call = Call::new("add", (a, b));
        let r: i32 = client.call(call).await.unwrap();
        assert_eq!((a + b), r);

        // add something with vec params
        let (a, b) = (2i32, 3i32);
        let call = Call::new("add", vec![a, b]);
        let r: i32 = client.call(call).await.unwrap();
        assert_eq!((a + b), r);

        // add something with array params
        let (a, b) = (2i32, 3i32);
        let call = Call::new("add", [a, b]);
        let r: i32 = client.call(call).await.unwrap();
        assert_eq!((a + b), r);

        // add something with slice params
        let ab = vec![2i32, 3i32];
        let call = Call::new("add", ab.as_slice());
        let r: i32 = client.call(call).await.unwrap();
        assert_eq!((a + b), r);

        // argument number mismatch
        let (a, b, c) = (2i32, 3i32, 4i32);
        let call: Call<_, i32> = Call::new("add", (a, b, c));
        let _fault = client.call(call).await.unwrap_err().downcast::<Fault>().unwrap();

        // argument type mismatch
        let (a, b) = ("12", "24");
        let call: Call<_, i32> = Call::new("add", (a, b));
        let _fault = client.call(call).await.unwrap_err().downcast::<Fault>().unwrap();
    };

    tokio::spawn(calls()).await.unwrap();

    trigger.notify_one();
    serve.await.unwrap().unwrap();
}
