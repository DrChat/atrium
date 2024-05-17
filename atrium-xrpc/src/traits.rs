use crate::error::Error;
use crate::error::{XrpcError, XrpcErrorKind};
use crate::types::{InputDataOrBytes, OutputDataOrBytes, XrpcRequest};
use async_trait::async_trait;
use http::{Method, Request, Response};
use serde::{de::DeserializeOwned, Serialize};

/// An abstract HTTP client.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HttpClient {
    async fn send_http(
        &self,
        request: Request<Vec<u8>>,
    ) -> core::result::Result<Response<Vec<u8>>, Box<dyn std::error::Error + Send + Sync + 'static>>;
}

type XrpcResult<O, E> = core::result::Result<OutputDataOrBytes<O>, self::Error<E>>;

/// An abstract XRPC client.
///
/// [`send_xrpc()`](XrpcClient::send_xrpc) method has a default implementation,
/// which wraps the [`HttpClient::send_http()`]` method to handle input and output as an XRPC Request.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait XrpcClient: HttpClient {
    fn base_uri(&self) -> String;
    #[allow(unused_variables)]
    async fn auth(&self, is_refresh: bool) -> Option<String> {
        None
    }
    async fn headers(&self) -> Vec<(String, String)> {
        Vec::new()
    }
    async fn send_xrpc<P, I, O, E>(&self, request: &XrpcRequest<P, I>) -> XrpcResult<O, E>
    where
        P: Serialize + Send + Sync,
        I: Serialize + Send + Sync,
        O: DeserializeOwned + Send + Sync,
        E: DeserializeOwned + Send + Sync,
    {
        let mut uri = format!("{}/xrpc/{}", self.base_uri(), request.path);
        if let Some(p) = &request.parameters {
            serde_html_form::to_string(p).map(|qs| {
                uri += "?";
                uri += &qs;
            })?;
        };
        let mut builder = Request::builder().method(&request.method).uri(&uri);
        if let Some(encoding) = &request.encoding {
            builder = builder.header(http::header::CONTENT_TYPE, encoding);
        }
        if let Some(token) = self
            .auth(
                request.method == Method::POST
                    && request.path == "com.atproto.server.refreshSession",
            )
            .await
        {
            builder = builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token));
        }
        for (key, value) in self.headers().await {
            builder = builder.header(key, value);
        }
        let body = if let Some(input) = &request.input {
            match input {
                InputDataOrBytes::Data(data) => serde_json::to_vec(&data)?,
                InputDataOrBytes::Bytes(bytes) => bytes.clone(),
            }
        } else {
            Vec::new()
        };
        let (parts, body) = self
            .send_http(builder.body(body)?)
            .await
            .map_err(Error::HttpClient)?
            .into_parts();
        if parts.status.is_success() {
            if parts
                .headers
                .get(http::header::CONTENT_TYPE)
                .and_then(|value| value.to_str().ok())
                .map_or(false, |content_type| {
                    content_type.starts_with("application/json")
                })
            {
                Ok(OutputDataOrBytes::Data(serde_json::from_slice(&body)?))
            } else {
                Ok(OutputDataOrBytes::Bytes(body))
            }
        } else {
            Err(Error::XrpcResponse(XrpcError {
                status: parts.status,
                error: serde_json::from_slice::<XrpcErrorKind<E>>(&body).ok(),
            }))
        }
    }
}
