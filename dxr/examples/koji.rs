// TODO: move code from this file to koji-rs

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::Client;
use url::Url;

use dxr::{Fault, FaultResponse, FromValue, MethodCall, MethodResponse, Value};

#[derive(Debug, FromValue)]
pub struct Build {
    pub build_id: i32,
    //cg_id: Option<?>,
    pub completion_time: String,
    pub completion_ts: f64,
    pub creation_event_id: i32,
    pub creation_time: String,
    pub creation_ts: f64,
    //pub epoch: Option<i32>, FIXME
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
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("text/xml").unwrap());

    let client = Client::builder().default_headers(headers).build().unwrap();
    let url = Url::parse("https://koji.fedoraproject.org/kojihub/").unwrap();

    // construct getBuild(nvr) method call
    let request = MethodCall::new(
        String::from("getBuild"),
        vec![Value::string(String::from("syncthing-1.1.0-1.fc30"))],
    );

    // construct HTTP body and content-length header from request
    let body = [
        r#"<?xml version="1.0"?>"#,
        quick_xml::se::to_string(&request).unwrap().as_str(),
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
        .unwrap();

    let response = client.execute(request).await.unwrap();

    // deserialize xml-rpc method response
    let contents = response.text().await.unwrap();

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
    let build = Build::from_value(values.first().unwrap()).unwrap();

    // print query result
    println!("{:#?}", build);

    Ok(())
}
