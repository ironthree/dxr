//! This file implements a test that launches a simple echo server, which is then used for roundtrip
//! tests with different types of values, including custom structs.

use std::borrow::Cow;
use std::collections::HashMap;
use std::time::Duration;

use dxr::chrono::{NaiveDateTime, SubsecRound, Utc};
use dxr::{DxrError, TryFromValue, TryToParams, TryToValue, Value};
use dxr_client::{ClientBuilder, ClientError};
use dxr_server::{axum::http::HeaderMap, HandlerFn, HandlerResult, RouteBuilder, Server};

fn echo_handler(params: &[Value], _headers: HeaderMap) -> HandlerResult {
    Ok(params.try_to_value()?)
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
async fn echo_any() {
    let route = RouteBuilder::new()
        .set_path("/")
        .add_method("echo", Box::new(echo_handler as HandlerFn))
        .build();

    let mut server = Server::from_route(route);
    let trigger = server.shutdown_trigger();

    let serve = tokio::spawn(server.serve("0.0.0.0:3000".parse().unwrap()));
    tokio::time::sleep(Duration::from_secs(1)).await;

    let calls = || async {
        let client = ClientBuilder::new("http://0.0.0.0:3000/".parse().unwrap())
            .user_agent("echo-client")
            .build();

        // i4
        let value = 42i32;
        let r: (i32,) = client.call("echo", (value,)).await.unwrap();
        assert_eq!((value,), r);

        // i8
        let value = 42i64;
        let r: (i64,) = client.call("echo", (value,)).await.unwrap();
        assert_eq!((value,), r);

        // double
        let value = 1.5f64;
        let r: (f64,) = client.call("echo", (value,)).await.unwrap();
        assert_eq!((value,), r);

        // boolean
        let value = true;
        let r: (bool,) = client.call("echo", (value,)).await.unwrap();
        assert_eq!((value,), r);

        // string
        let value = String::from("HELLO WORLD");
        let r: (String,) = client.call("echo", (value.as_str(),)).await.unwrap();
        assert_eq!((value,), r);

        // datetime
        let value = Utc::now().round_subsecs(0).naive_utc();
        let r: (NaiveDateTime,) = client.call("echo", (value,)).await.unwrap();
        assert_eq!((value,), r);

        // bytes
        let value = b"HELLOWORLD".to_vec();
        let r: (Vec<u8>,) = client.call("echo", (value.as_slice(),)).await.unwrap();
        assert_eq!((value,), r);

        // array
        let value = vec![-12i32, 42i32];
        let r: Vec<i32> = client.call("echo", value.as_slice()).await.unwrap();
        assert_eq!(value, r);

        // option
        let value = Some(42i32);
        let r: (Option<i32>,) = client.call("echo", (value,)).await.unwrap();
        assert_eq!((value,), r);

        // map
        let mut value: HashMap<String, Value> = HashMap::new();
        value.insert(String::from("foo"), Value::i4(21));
        value.insert(String::from("bar"), Value::i8(42));
        let r: (HashMap<String, Value>,) = client.call("echo", (value.clone(),)).await.unwrap();
        assert_eq!((value,), r);

        // struct
        let value = TestStruct {
            integer: 21,
            long: 42,
            string: String::from("HELLO WORLD!"),
            double: 2.5,
            list: vec![1.5, 2.5],
            option: Some(-21),
        };
        let r: (TestStruct,) = client.call("echo", (value.clone(),)).await.unwrap();
        assert_eq!((value,), r);

        // tuples
        let value = (1, 2);
        type Pair = (i32, i32);
        let r: Pair = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        let value = (1, 2, 3);
        type Triple = (i32, i32, i32);
        let r: Triple = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        let value = (1, 2, 3, 4);
        type Quadruple = (i32, i32, i32, i32);
        let r: Quadruple = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        let value = (1, 2, 3, 4, 5);
        type Quintuple = (i32, i32, i32, i32, i32);
        let r: Quintuple = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        let value = (1, 2, 3, 4, 5, 6);
        type Sextuple = (i32, i32, i32, i32, i32, i32);
        let r: Sextuple = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        let value = (1, 2, 3, 4, 5, 6, 7);
        type Septuple = (i32, i32, i32, i32, i32, i32, i32);
        let r: Septuple = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        let value = (1, 2, 3, 4, 5, 6, 7, 8);
        type Octuple = (i32, i32, i32, i32, i32, i32, i32, i32);
        let r: Octuple = client.call("echo", value).await.unwrap();
        assert_eq!(value, r);

        // missing field
        {
            #[derive(TryToValue)]
            struct Params {
                foo: i32,
                bar: i32,
            }
            #[allow(dead_code)]
            #[derive(TryFromValue, Debug)]
            struct Response {
                foo: i32,
                baz: i32,
            }
            let value = Params { foo: 1, bar: 2 };
            assert!(matches!(
                client.call::<_, (Response,)>("echo", (value,)).await.unwrap_err(),
                ClientError::RPC {
                    error: DxrError::MissingField {
                        name: Cow::Borrowed("Response"),
                        field: Cow::Borrowed("baz")
                    }
                }
            ));
        }

        // escaped field in params
        {
            #[derive(TryToValue)]
            struct Params {
                r#foo: i32,
            }
            #[derive(Eq, PartialEq, TryFromValue, Debug)]
            struct Response {
                foo: i32,
            }
            let value = Params { foo: 1 };
            let r: (Response,) = client.call("echo", (value,)).await.unwrap();
            assert_eq!(r, (Response { foo: 1 },));
        }

        // escaped field in response
        {
            #[derive(TryToValue)]
            struct Params {
                foo: i32,
            }
            #[derive(Eq, PartialEq, TryFromValue, Debug)]
            struct Response {
                r#foo: i32,
            }
            let value = Params { foo: 1 };
            let r: (Response,) = client.call("echo", (value,)).await.unwrap();
            assert_eq!(r, (Response { foo: 1 },));
        }

        // escaped field in params & response
        {
            #[derive(TryToValue)]
            struct Params {
                r#type: i32,
            }
            #[derive(Eq, PartialEq, TryFromValue, Debug)]
            struct Response {
                r#type: i32,
            }
            let value = Params { r#type: 1 };
            let r: (Response,) = client.call("echo", (value,)).await.unwrap();
            assert_eq!(r, (Response { r#type: 1 },));
        }

        // type mismatch
        let value = -12i32;
        assert!(matches!(
            client.call::<(i32,), (String,)>("echo", (value,)).await.unwrap_err(),
            ClientError::RPC {
                error: DxrError::WrongType { .. }
            }
        ));

        // parameter number mismatch
        let value = vec![2i32, 3i32];
        assert!(matches!(
            client
                .call::<Vec<i32>, (i32, i32, i32)>("echo", value)
                .await
                .unwrap_err(),
            ClientError::RPC {
                error: DxrError::ParameterMismatch { .. }
            }
        ));

        // multicall
        let calls = vec![
            ("echo".into(), (-12i32).try_to_params().unwrap()),
            ("echo".into(), (0, 1, 2).try_to_params().unwrap()),
        ];
        let r = client.multicall(calls).await.unwrap();
        assert_eq!(
            r,
            vec![
                Ok(vec![-12].try_to_value().unwrap()),
                Ok(vec![0, 1, 2].try_to_value().unwrap()),
            ],
        );
    };

    tokio::spawn(calls()).await.unwrap();

    trigger.notify_one();
    serve.await.unwrap().unwrap();
}
