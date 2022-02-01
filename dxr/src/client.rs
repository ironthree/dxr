use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
use url::Url;

use crate::call::Call;
use crate::types::{Fault, FaultResponse, MethodResponse};
use crate::{DxrError, FromDXR, ToDXR};

#[derive(Debug)]
pub struct Client {
    url: Url,
}

impl Client {
    pub fn new(url: Url) -> Client {
        Client { url }
    }
}

impl Client {
    pub async fn call<P: ToDXR, R: FromDXR>(&self, call: Call<P, R>) -> Result<R, DxrError> {
        // default headers for xml-rpc calls
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("text/xml").expect("Failed to parse hardcoded Content-Type header."),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to initialize reqwest client.");

        //let url = Url::parse("https://koji.fedoraproject.org/kojihub/").expect("Failed to parse hardcoded URL.");
        let request = call.params_to_dxr()?;

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
            .post(self.url.clone())
            .body(body)
            .header(CONTENT_LENGTH, HeaderValue::from(content_length))
            .build()
            .expect("Failed to construct POST request.");

        let response = client.execute(request).await.expect("Network request failed.");

        // deserialize xml-rpc method response
        let contents = response.text().await.expect("Failed to decode response body.");

        let response: MethodResponse = match quick_xml::de::from_str(&contents) {
            Ok(response) => response,
            Err(error1) => {
                let fault: Fault = match quick_xml::de::from_str(&contents) {
                    Ok(fault) => FaultResponse::into(fault),
                    Err(error2) => {
                        log::error!("Failed to deserialize response:");
                        log::error!("  Response failed with: {}", error1);
                        log::error!("  Fault failed with: {}", error2);
                        return Err(DxrError::invalid_data(contents));
                    },
                };

                return Err(DxrError::server_fault(fault));
            },
        };

        R::from_dxr(&response.inner())
    }
}
