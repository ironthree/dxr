// TODO: move code from this file to koji-rs

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::Client;
use url::Url;

use dxr_shared::{Fault, FaultResponse, MethodCall, MethodResponse, Value};

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

    let build: MethodResponse = match quick_xml::de::from_str(&contents) {
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

    // print query result
    println!("{:#?}", build);

    Ok(())
}
