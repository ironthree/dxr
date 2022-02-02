use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use url::Url;

use dxr_shared::{DxrError, FaultResponse, FromDXR, MethodCall, MethodResponse, ToParams};

use crate::call::Call;

const DEFAULT_USER_AGENT: &str = concat!("DXR Client v", env!("CARGO_PKG_VERSION"));

/// data type that contains all values that are required for constructing a [`Client`]
#[derive(Debug)]
pub struct ClientBuilder {
    url: Url,
    headers: HeaderMap,
    user_agent: Option<&'static str>,
}

impl ClientBuilder {
    /// constructor for [`ClientBuilder`] from the URL of the XML-RPC server
    pub fn new(url: Url) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/xml"));

        ClientBuilder {
            url,
            headers: default_headers,
            user_agent: None,
        }
    }

    /// method for overriding the default User-Agent header
    pub fn user_agent(mut self, user_agent: &'static str) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    fn add_header(&mut self, name: HeaderName, value: HeaderValue) {
        self.headers.insert(name, value);
    }

    /// build the [`Client`] by setting up and initializing the internal [`reqwest::Client`]
    pub fn build(mut self) -> Client {
        self.add_header(
            USER_AGENT,
            HeaderValue::from_static(self.user_agent.unwrap_or(DEFAULT_USER_AGENT)),
        );

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

/// # XML-RPC client implementation
///
/// This type provides a very simple XML-RPC client implementation. Initialize the [`Client`],
/// submit a [`Call`], get a result (or a fault).
#[derive(Debug)]
pub struct Client {
    url: Url,
    client: reqwest::Client,
}

impl Client {
    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// asynchronous method for handling remote procedure calls with XML-RPC
    ///
    /// Fault responses from the XML-RPC server are transparently converted into
    /// [`DxrError::ServerFault`] errors.
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
