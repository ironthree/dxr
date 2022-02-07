//! This file implements a test that launches a simple echo server, which is then used for roundtrip
//! tests with single values, including custom structs.

use std::collections::HashMap;
use std::time::Duration;

use dxr::axum::http::HeaderMap;
use dxr::chrono::{DateTime, SubsecRound, Utc};
use dxr::{Call, ClientBuilder, Fault, FromDXR, FromParams, HandlerFn, ServerBuilder, ToDXR, Value};

fn echo_handler(params: &[Value], _headers: &HeaderMap) -> Result<Value, Fault> {
    let value: Value = Value::from_params(params)?;
    Ok(value.to_dxr()?)
}

#[derive(Clone, Debug, FromDXR, PartialEq, ToDXR)]
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
    let server = ServerBuilder::new("0.0.0.0:3000".parse().unwrap())
        .add_method("echo", Box::new(echo_handler as HandlerFn))
        .build();

    let serve = tokio::spawn(server.serve());
    tokio::time::sleep(Duration::from_secs(1)).await;

    let calls = || async {
        let client = ClientBuilder::new("http://0.0.0.0:3000".parse().unwrap())
            .user_agent("echo-client")
            .build();

        // i4
        let value = 42i32;
        let call: Call<_, i32> = Call::new("echo", value);
        let r = client.call(call).await.unwrap();
        assert_eq!(value, r);

        // i8
        let value = 42i64;
        let call: Call<_, i64> = Call::new("echo", value);
        let r = client.call(call).await.unwrap();
        assert_eq!(value, r);

        // double
        let value = 1.5f64;
        let call: Call<_, f64> = Call::new("echo", value);
        let r = client.call(call).await.unwrap();
        assert_eq!(value, r);

        // boolean
        let value = true;
        let call: Call<_, bool> = Call::new("echo", value);
        let r = client.call(call).await.unwrap();
        assert_eq!(value, r);

        // string
        let value = String::from("HELLO WORLD");
        let call: Call<_, String> = Call::new("echo", value.as_str());
        let r = client.call(call).await.unwrap();
        assert_eq!(value, r);

        // datetime
        let value = Utc::now().round_subsecs(0);
        let call: Call<_, DateTime<Utc>> = Call::new("echo", value);
        let r = client.call(call).await.unwrap();
        assert_eq!(value, r);

        // bytes
        let value = b"HELLOWORLD".to_vec();
        let call: Call<_, Vec<u8>> = Call::new("echo", value.as_slice());
        let r = client.call(call).await.unwrap();
        assert_eq!(value, r);

        // array
        let value = vec![vec![-12i32, 42i32]];
        let call: Call<_, Vec<i32>> = Call::new("echo", value.as_slice());
        let r = client.call(call).await.unwrap();
        assert_eq!(value, vec![r]);

        // option
        let value = Some(42i32);
        let call: Call<_, Option<i32>> = Call::new("echo", value);
        let r = client.call(call).await.unwrap();
        assert_eq!(value, r);

        // map
        let mut value: HashMap<String, Value> = HashMap::new();
        value.insert(String::from("foo"), Value::i4(21));
        value.insert(String::from("bar"), Value::i8(42));
        let call: Call<_, HashMap<String, Value>> = Call::new("echo", value.clone());
        let r = client.call(call).await.unwrap();
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
        let call: Call<_, TestStruct> = Call::new("echo", vec![value.clone()]);
        let r = client.call(call).await.unwrap();
        assert_eq!(value, r);

        // type mismatch
        let value = -12i32;
        let call: Call<i32, String> = Call::new("echo", value);
        assert!(client.call(call).await.unwrap_err().is_wrong_type());

        // server-side parameter number mismatch
        let value = vec![-12i32, 42i32];
        let call: Call<_, Vec<i32>> = Call::new("echo", value);
        assert!(client.call(call).await.unwrap_err().is_server_fault());
    };

    tokio::spawn(calls()).await.unwrap();

    serve.abort();
    assert!(serve.await.unwrap_err().is_cancelled());
}
