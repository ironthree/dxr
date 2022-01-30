// TODO: move code from this file to koji-rs

#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::Client;
use url::Url;

use dxr::types::{Fault, FaultResponse, MethodCall, MethodResponse, Value};
use dxr::{FromDXR, ToDXR};

#[derive(Debug, FromDXR, ToDXR)]
pub struct Build {
    pub build_id: i32,
    //cg_id: Option<?>,
    pub completion_time: String,
    pub completion_ts: f64,
    pub creation_event_id: i32,
    pub creation_time: String,
    pub creation_ts: f64,
    pub epoch: Option<i32>,
    //extra: HashMap<String, Value>,
    pub id: i32,
    pub name: String,
    pub nvr: String,
    pub owner_id: i32,
    pub owner_name: String,
    pub package_id: i32,
    pub package_name: String,
    pub release: String,
    pub source: String,
    pub start_time: String,
    pub start_ts: f64,
    pub state: i32,
    pub task_id: i32,
    pub version: String,
    pub volume_id: i32,
    pub volume_name: String,
    //cg_name: Option<?>,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    // default headers for xml-rpc calls
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("text/xml").expect("Failed to parse hardcoded Content-Type header."),
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to initialize reqwest client.");
    let url = Url::parse("https://koji.fedoraproject.org/kojihub/").expect("Failed to parse hardcoded URL.");

    // construct getBuild(nvr) method call
    let request = MethodCall::new(
        String::from("getBuild"),
        vec![Value::string(String::from("syncthing-1.1.0-1.fc30"))],
    );

    // construct HTTP body and content-length header from request
    let body = [
        r#"<?xml version="1.0"?>"#,
        quick_xml::se::to_string(&request)
            .expect("Failed to serialize XML-RPC request.")
            .as_str(),
        "",
    ]
    .join("\n");
    let content_length = body.as_bytes().len();

    // construct request and send to server
    let request = client
        .post(url)
        .body(body)
        .header(CONTENT_LENGTH, HeaderValue::from(content_length))
        .build()
        .expect("Failed to construct POST request.");

    let response = client.execute(request).await.expect("Network request failed.");

    // deserialize xml-rpc method response
    let contents = response.text().await.expect("Failed to decode response body.");

    let response: MethodResponse = match quick_xml::de::from_str(&contents) {
        Ok(build) => build,
        Err(error1) => {
            let fault: Fault = match quick_xml::de::from_str(&contents) {
                Ok(fault) => FaultResponse::into(fault),
                Err(error2) => {
                    eprintln!("Failed to deserialize response:");
                    eprintln!("  Response failed with: {}", error1);
                    eprintln!("  Fault failed with: {}", error2);
                    return Err(contents);
                },
            };

            return Err(fault.to_string());
        },
    };

    let values = response.into_values();
    let build = Build::from_dxr(values.first().expect("Failed to get one value from the response."))
        .expect("Failed to deserialize XML-RPC response into a Build.");

    // print query result
    println!("{:#?}", build);

    Ok(())
}
