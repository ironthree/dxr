use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use url::Url;

use dxr_shared::{DxrError, FaultResponse, FromDXR, MethodCall, MethodResponse, ToParams};

mod call;
pub use call::*;

/// default value of the `User-Agent` HTTP header for XML-RPC requests
#[cfg_attr(docsrs, doc(cfg(feature = "client")))]
pub const DEFAULT_USER_AGENT: &str = concat!("dxr-client-v", env!("CARGO_PKG_VERSION"));

/// builder that takes parameters for constructing a [`Client`]
#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "client")))]
pub struct ClientBuilder {
    url: Url,
    headers: HeaderMap,
    user_agent: Option<&'static str>,
}

impl ClientBuilder {
    /// constructor for [`ClientBuilder`] from the URL of the XML-RPC server
    ///
    /// This also sets up the default `Content-Type: text/xml` HTTP header for XML-RPC requests.
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

    /// method for providing additional custom HTTP headers
    ///
    /// Using [`HeaderName`] constants for the header name is recommended. The [`HeaderValue`]
    /// argument needs to be parsed (probably from a string) with [`HeaderValue::from_str`] to
    /// ensure their value is valid.
    pub fn add_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(name, value);
        self
    }

    /// build the [`Client`] by setting up and initializing the internal [`reqwest::Client`]
    ///
    /// If no custom value was provided for `User-Agent`, the default value
    /// ([`DEFAULT_USER_AGENT`]) will be used.
    pub fn build(self) -> Client {
        let user_agent = self.user_agent.unwrap_or(DEFAULT_USER_AGENT);

        let builder = self.add_header(USER_AGENT, HeaderValue::from_static(user_agent));

        let client = reqwest::Client::builder()
            .default_headers(builder.headers)
            .build()
            .expect("Failed to initialize reqwest client.");

        Client {
            url: builder.url,
            client,
        }
    }
}

fn request_to_body(call: &MethodCall) -> Result<String, DxrError> {
    let body = [
        r#"<?xml version="1.0"?>"#,
        quick_xml::se::to_string(&call)
            .map_err(|error| DxrError::invalid_data(error.to_string()))?
            .as_str(),
        "",
    ]
    .join("\n");

    Ok(body)
}

fn request_to_result(contents: &str) -> Result<MethodResponse, DxrError> {
    // need to check for FaultResponse first:
    // - a missing <params> tag is ambiguous (can be either an empty response, or a fault response)
    // - a present <fault> tag is unambiguous
    let error2 = match quick_xml::de::from_str(contents) {
        Ok(fault) => return Err(DxrError::server_fault(FaultResponse::try_into(fault)?)),
        Err(error) => error.to_string(),
    };

    let error1 = match quick_xml::de::from_str(contents) {
        Ok(response) => return Ok(response),
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
#[cfg_attr(docsrs, doc(cfg(feature = "client")))]
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
    pub async fn call<P: ToParams, R: FromDXR>(&self, call: Call<'_, P, R>) -> Result<R, DxrError> {
        let request = call.as_xml_rpc()?;

        // construct HTTP body and content-length header from request
        let body = request_to_body(&request)?;

        // construct request and send to server
        let request = self
            .client()
            .post(self.url.clone())
            .body(body)
            .build()
            .expect("Failed to construct POST request.");

        let response = self.client().execute(request).await.expect("Network request failed.");

        // deserialize xml-rpc method response
        let contents = response.text().await.expect("Failed to decode response body.");
        let result = request_to_result(&contents)?;

        if let Some(value) = result.inner() {
            R::from_dxr(&value)
        } else {
            #[cfg(feature = "nil")]
            {
                use dxr_shared::Value;
                R::from_dxr(&Value::nil())
            }

            #[cfg(not(feature = "nil"))]
            {
                Err(DxrError::parameter_mismatch(0, 1))
            }
        }
    }
}
