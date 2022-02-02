use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use url::Url;

use dxr_shared::{DxrError, FaultResponse, FromDXR, MethodCall, MethodResponse, ToParams};

use crate::call::Call;

#[derive(Debug)]
pub struct ClientBuilder {
    url: Url,
    headers: HeaderMap,
}

impl ClientBuilder {
    pub fn new(url: Url) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/xml"));

        ClientBuilder {
            url,
            headers: default_headers,
        }
    }

    pub fn user_agent(self, user_agent: &'static str) -> Self {
        self.add_header(USER_AGENT, HeaderValue::from_static(user_agent))
    }

    fn add_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(name, value);
        self
    }

    pub fn build(self) -> Client {
        let client = reqwest::Client::builder()
            .default_headers(self.headers)
            .build()
            .expect("Failed to initialize reqwest client.");

        Client { url: self.url, client }
    }
}

fn request_to_body(call: &MethodCall) -> Result<(String, usize), DxrError> {
    let body = [
        r#"<?xml version="1.0"?>"#,
        quick_xml::se::to_string(&call)
            .map_err(|error| DxrError::invalid_data(error.to_string()))?
            .as_str(),
        "",
    ]
    .join("\n");

    let content_length = body.as_bytes().len();

    Ok((body, content_length))
}

fn request_to_result(contents: &str) -> Result<MethodResponse, DxrError> {
    let error1 = match quick_xml::de::from_str(contents) {
        Ok(response) => return Ok(response),
        Err(error) => error.to_string(),
    };

    let error2 = match quick_xml::de::from_str(contents) {
        Ok(fault) => return Err(DxrError::server_fault(FaultResponse::into(fault))),
        Err(error) => error.to_string(),
    };

    log::debug!("Failed to deserialize response as either value or fault.");
    log::debug!("Response failed with: {}; Fault failed with: {}", error1, error2);

    Err(DxrError::invalid_data(contents.to_owned()))
}

#[derive(Debug)]
pub struct Client {
    url: Url,
    client: reqwest::Client,
}

impl Client {
    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    pub async fn call<P: ToParams, R: FromDXR>(&self, call: Call<P, R>) -> Result<R, DxrError> {
        let request = call.as_xml_rpc()?;

        // construct HTTP body and content-length header from request
        let (body, content_length) = request_to_body(&request)?;

        // construct request and send to server
        let request = self
            .client()
            .post(self.url.clone())
            .body(body)
            .header(CONTENT_LENGTH, HeaderValue::from(content_length))
            .build()
            .expect("Failed to construct POST request.");

        let response = self.client().execute(request).await.expect("Network request failed.");

        // deserialize xml-rpc method response
        let contents = response.text().await.expect("Failed to decode response body.");
        let result = request_to_result(&contents)?;

        R::from_dxr(&result.inner())
    }
}
