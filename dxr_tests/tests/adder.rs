//! This file implements a test that launches a simple server that can add integers.

use std::time::Duration;

use dxr::{Fault, TryFromParams, TryToValue, Value};
use dxr_client::{Call, ClientBuilder, ClientError};
use dxr_server::{axum::http::HeaderMap, HandlerFn, HandlerResult, RouteBuilder, Server};

fn adder_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    let (a, b): (i32, i32) = TryFromParams::try_from_params(params)?;
    Ok((a + b).try_to_value()?)
}

#[tokio::test]
async fn adder() {
    let route = RouteBuilder::new()
        .set_path("/")
        .add_method("add", Box::new(adder_handler as HandlerFn))
        .build();

    let mut server = Server::from_route(route);
    let trigger = server.shutdown_trigger();

    let serve = tokio::spawn(server.serve("0.0.0.0:3000".parse().unwrap()));
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

        // multicall
        let call = Call::multicall(vec![
            (String::from("add"), (1, 2)),
            (String::from("add"), (-3, -5)),
            (String::from("add"), (-1, 1)),
            (String::from("sub"), (1, 2)),
        ])
        .unwrap();
        let values = client.multicall(call).await.unwrap();
        assert_eq!(
            values,
            vec![
                Ok(Value::i4(3)),
                Ok(Value::i4(-8)),
                Ok(Value::i4(0)),
                Err(Fault::new(404, String::from("Unknown method.")))
            ]
        );

        // argument number mismatch
        let (a, b, c) = (2i32, 3i32, 4i32);
        let call: Call<_, i32> = Call::new("add", (a, b, c));
        assert!(matches!(
            client.call(call).await.unwrap_err(),
            ClientError::Fault { .. }
        ));

        // argument type mismatch
        let (a, b) = ("12", "24");
        let call: Call<_, i32> = Call::new("add", (a, b));
        assert!(matches!(
            client.call(call).await.unwrap_err(),
            ClientError::Fault { .. }
        ));
    };

    tokio::spawn(calls()).await.unwrap();

    trigger.notify_one();
    serve.await.unwrap().unwrap();
}
