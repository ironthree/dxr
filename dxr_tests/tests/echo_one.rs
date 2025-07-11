//! This file implements a test that launches a simple echo server, which is then used for roundtrip
//! tests with single values, including custom structs.

use std::collections::HashMap;
use std::time::Duration;

use dxr::{DxrError, TryFromParams, TryFromValue, TryToParams, TryToValue, Value};
use dxr_client::{ClientBuilder, ClientError};
use dxr_server::{axum::http::HeaderMap, HandlerFn, HandlerResult, RouteBuilder, Server};

use chrono::{NaiveDateTime, SubsecRound, Utc};

fn echo_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    let value: Value = Value::try_from_params(params)?;
    Ok(value.try_to_value()?)
}

#[derive(Clone, Debug, TryFromValue, TryToValue, PartialEq)]
struct TestStruct {
    integer: i32,
    long: i64,
    string: String,
    double: f64,
    list: Vec<f64>,
    option: Option<i32>,
}

#[tokio::test]
async fn echo_one() {
    let route = RouteBuilder::new()
        .set_path("/")
        .add_method("echo", Box::new(echo_handler as HandlerFn))
        .build();

    let mut server = Server::from_route(route);
    let trigger = server.shutdown_trigger();

    let serve = tokio::spawn(server.serve("0.0.0.0:3000".parse().unwrap()));
    tokio::time::sleep(Duration::from_secs(1)).await;

    let calls = || async {
        let client = ClientBuilder::new("http://0.0.0.0:3000".parse().unwrap())
            .user_agent("echo-client")
            .build();

        // i4
        let value = 42i32;
        let r: i32 = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        // i8
        let value = 42i64;
        let r: i64 = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        // double
        let value = 1.5f64;
        let r: f64 = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        // boolean
        let value = true;
        let r: bool = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        // string
        let value = String::from("HELLO WORLD");
        let r: String = client.call("echo", value.as_str()).await.unwrap();
        assert_eq!(value, r);

        // datetime
        let value = Utc::now().round_subsecs(0).naive_utc();
        let r: NaiveDateTime = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        // bytes
        let value = b"HELLOWORLD".to_vec();
        let r: Vec<u8> = client.call("echo", value.as_slice()).await.unwrap();
        assert_eq!(value, r);

        // array
        let value = vec![vec![-12i32, 42i32]];
        let r: Vec<i32> = client.call("echo", value.as_slice()).await.unwrap();
        assert_eq!(value, vec![r]);

        // option
        let value = Some(42i32);
        let r: Option<i32> = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        // map
        let mut value: HashMap<String, Value> = HashMap::new();
        value.insert(String::from("foo"), Value::i4(21));
        value.insert(String::from("bar"), Value::i8(42));
        let r: HashMap<String, Value> = client.call("echo", value.clone()).await.unwrap();
        assert_eq!(value, r);

        // struct
        let value = TestStruct {
            integer: 21,
            long: 42,
            string: String::from("HELLO WORLD!"),
            double: 2.5,
            list: vec![1.5, 2.5],
            option: Some(-21),
        };
        let r: TestStruct = client.call("echo", vec![value.clone()]).await.unwrap();
        assert_eq!(value, r);

        // type mismatch
        let value = -12i32;
        assert!(matches!(
            client.call::<i32, String>("echo", value).await.unwrap_err(),
            ClientError::RPC {
                error: DxrError::WrongType { .. }
            }
        ));

        // server-side parameter number mismatch
        let value = vec![-12i32, 42i32];
        assert!(matches!(
            client.call::<Vec<i32>, Vec<i32>>("echo", value).await.unwrap_err(),
            ClientError::Fault { .. }
        ));

        // multicall
        let calls = vec![
            ("echo".into(), (-12i32).try_to_params().unwrap()),
            ("echo".into(), 42.try_to_params().unwrap()),
        ];
        let r = client.multicall(calls).await.unwrap();
        assert_eq!(r, vec![Ok(Value::i4(-12)), Ok(Value::i4(42))]);
    };

    tokio::spawn(calls()).await.unwrap();

    trigger.notify_one();
    serve.await.unwrap().unwrap();
}
