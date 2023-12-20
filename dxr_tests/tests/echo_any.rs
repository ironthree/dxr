//! This file implements a test that launches a simple echo server, which is then used for roundtrip
//! tests with different types of values, including custom structs.

use std::borrow::Cow;
use std::collections::HashMap;
use std::time::Duration;

use dxr::chrono::{NaiveDateTime, SubsecRound, Utc};
use dxr::{DxrError, TryFromValue, TryToValue, Value};
use dxr_client::{Call, ClientBuilder, ClientError};
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
async fn echo() {
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
        let call = Call::new("echo", (value,));
        let r: (i32,) = client.call(call).await.unwrap();
        assert_eq!((value,), r);

        // i8
        let value = 42i64;
        let call = Call::new("echo", (value,));
        let r: (i64,) = client.call(call).await.unwrap();
        assert_eq!((value,), r);

        // double
        let value = 1.5f64;
        let call = Call::new("echo", (value,));
        let r: (f64,) = client.call(call).await.unwrap();
        assert_eq!((value,), r);

        // boolean
        let value = true;
        let call = Call::new("echo", (value,));
        let r: (bool,) = client.call(call).await.unwrap();
        assert_eq!((value,), r);

        // string
        let value = String::from("HELLO WORLD");
        let call = Call::new("echo", (value.as_str(),));
        let r: (String,) = client.call(call).await.unwrap();
        assert_eq!((value,), r);

        // datetime
        let value = Utc::now().round_subsecs(0).naive_utc();
        let call = Call::new("echo", (value,));
        let r: (NaiveDateTime,) = client.call(call).await.unwrap();
        assert_eq!((value,), r);

        // bytes
        let value = b"HELLOWORLD".to_vec();
        let call = Call::new("echo", (value.as_slice(),));
        let r: (Vec<u8>,) = client.call(call).await.unwrap();
        assert_eq!((value,), r);

        // array
        let value = vec![-12i32, 42i32];
        let call = Call::new("echo", value.as_slice());
        let r: Vec<i32> = client.call(call).await.unwrap();
        assert_eq!(value, r);

        // option
        let value = Some(42i32);
        let call = Call::new("echo", (value,));
        let r: (Option<i32>,) = client.call(call).await.unwrap();
        assert_eq!((value,), r);

        // map
        let mut value: HashMap<String, Value> = HashMap::new();
        value.insert(String::from("foo"), Value::i4(21));
        value.insert(String::from("bar"), Value::i8(42));
        let call = Call::new("echo", (value.clone(),));
        let r: (HashMap<String, Value>,) = client.call(call).await.unwrap();
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
        let call = Call::new("echo", (value.clone(),));
        let r: (TestStruct,) = client.call(call).await.unwrap();
        assert_eq!((value,), r);

        // tuples
        let value = (1, 2);
        type Pair = (i32, i32);
        let call = Call::new("echo", value);
        let r: Pair = client.call(call).await.unwrap();
        assert_eq!(value, r);

        let value = (1, 2, 3);
        type Triple = (i32, i32, i32);
        let call = Call::new("echo", value);
        let r: Triple = client.call(call).await.unwrap();
        assert_eq!(value, r);

        let value = (1, 2, 3, 4);
        type Quadruple = (i32, i32, i32, i32);
        let call = Call::new("echo", value);
        let r: Quadruple = client.call(call).await.unwrap();
        assert_eq!(value, r);

        let value = (1, 2, 3, 4, 5);
        type Quintuple = (i32, i32, i32, i32, i32);
        let call = Call::new("echo", value);
        let r: Quintuple = client.call(call).await.unwrap();
        assert_eq!(value, r);

        let value = (1, 2, 3, 4, 5, 6);
        type Sextuple = (i32, i32, i32, i32, i32, i32);
        let call = Call::new("echo", value);
        let r: Sextuple = client.call(call).await.unwrap();
        assert_eq!(value, r);

        let value = (1, 2, 3, 4, 5, 6, 7);
        type Septuple = (i32, i32, i32, i32, i32, i32, i32);
        let call = Call::new("echo", value);
        let r: Septuple = client.call(call).await.unwrap();
        assert_eq!(value, r);

        let value = (1, 2, 3, 4, 5, 6, 7, 8);
        type Octuple = (i32, i32, i32, i32, i32, i32, i32, i32);
        let call = Call::new("echo", value);
        let r: Octuple = client.call(call).await.unwrap();
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
            let call: Call<'_, _, (Response,)> = Call::new("echo", (value,));
            assert!(matches!(
                client.call(call).await.unwrap_err(),
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
            let call = Call::new("echo", (value,));
            let r: (Response,) = client.call(call).await.unwrap();
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
            let call = Call::new("echo", (value,));
            let r: (Response,) = client.call(call).await.unwrap();
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
            let call = Call::new("echo", (value,));
            let r: (Response,) = client.call(call).await.unwrap();
            assert_eq!(r, (Response { r#type: 1 },));
        }

        // type mismatch
        let value = -12i32;
        let call: Call<(i32,), (String,)> = Call::new("echo", (value,));
        assert!(matches!(
            client.call(call).await.unwrap_err(),
            ClientError::RPC {
                error: DxrError::WrongType { .. }
            }
        ));

        // parameter number mismatch
        let value = vec![2i32, 3i32];
        let call: Call<Vec<i32>, (i32, i32, i32)> = Call::new("echo", value);
        assert!(matches!(
            client.call(call).await.unwrap_err(),
            ClientError::RPC {
                error: DxrError::ParameterMismatch { .. }
            }
        ));
    };

    tokio::spawn(calls()).await.unwrap();

    trigger.notify_one();
    serve.await.unwrap().unwrap();
}
