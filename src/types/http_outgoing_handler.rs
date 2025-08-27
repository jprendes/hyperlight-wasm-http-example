use crate::bindings::wasi::{self};
use crate::resource::{BlockOn, Resource};

use wasi::http::types::ErrorCode;

use super::WasiImpl;
use super::headers::Headers;
use super::http_future_incoming_response::FutureIncomingResponse;
use super::http_incoming_body::IncomingBody;
use super::http_incoming_response::IncomingResponse;
use super::http_outgoing_request::OutgoingRequest;
use super::http_request_options::RequestOptions;
use super::io_stream::Stream;

impl
    wasi::http::OutgoingHandler<
        ErrorCode,
        Resource<FutureIncomingResponse>,
        Resource<OutgoingRequest>,
        Resource<RequestOptions>,
    > for WasiImpl
{
    fn handle(
        &mut self,
        request: Resource<OutgoingRequest>,
        _options: Option<Resource<RequestOptions>>,
    ) -> Result<Resource<FutureIncomingResponse>, ErrorCode> {
        // TODO: honor the request options
        let request = request.read().block_on();
        let method = request.method.clone().try_into()?;
        let authority = request.authority.as_ref().map(|s| s.as_str()).unwrap_or("");
        let path_with_query = request.path_with_query.clone().unwrap_or_default();
        let scheme = request
            .scheme
            .clone()
            .unwrap_or(wasi::http::types::Scheme::HTTP);
        let scheme = scheme.to_string();
        let url = format!("{scheme}://{authority}/{path_with_query}");

        let headers = request.headers.read().block_on().entries();

        let mut builder = self.client.request(method, url);
        for (k, v) in headers {
            builder = builder.header(k, v);
        }

        let future_response = Resource::new(FutureIncomingResponse::default());
        let future_response_clone = future_response.clone();
        async move {
            let body = request.body.clone();
            let mut body = body.write_wait_until(|b| b.is_finished()).await;

            // TODO: actually use the trailers
            let _trailers = body.trailers.clone();

            // TODO: use a streaming body instead of reading it all at once
            let body = body.read_all().await;

            let builder = builder.body(body);

            let response = builder.send().await;

            let response = match response {
                Ok(resp) => resp,
                Err(err) => {
                    future_response_clone
                        .write()
                        .await
                        .set(Err(ErrorCode::InternalError(Some(err.to_string()))));
                    return;
                }
            };

            let status = response.status().as_u16();
            let mut headers: Headers = response.headers().clone().into();
            headers.immutable = true;

            let bytes = match response.bytes().await {
                Ok(bytes) => bytes,
                Err(err) => {
                    future_response_clone
                        .write()
                        .await
                        .set(Err(ErrorCode::InternalError(Some(err.to_string()))));
                    return;
                }
            };

            let mut stream = Stream::new();
            let _ = stream.write(bytes);
            let _ = stream.close();
            let body = IncomingBody {
                stream: Resource::new(stream),
                trailers: Resource::default(),
                stream_taken: false,
            };

            let response = IncomingResponse {
                status,
                headers: Resource::new(headers),
                body: Resource::new(body),
                body_taken: false,
            };

            future_response_clone
                .write()
                .await
                .set(Ok(Resource::new(response)));
        }
        .spawn();

        Ok(future_response)
    }
}

impl TryFrom<wasi::http::types::Method> for reqwest::Method {
    type Error = ErrorCode;
    fn try_from(value: wasi::http::types::Method) -> Result<Self, Self::Error> {
        match value {
            wasi::http::types::Method::Get => Ok(reqwest::Method::GET),
            wasi::http::types::Method::Post => Ok(reqwest::Method::POST),
            wasi::http::types::Method::Put => Ok(reqwest::Method::PUT),
            wasi::http::types::Method::Delete => Ok(reqwest::Method::DELETE),
            wasi::http::types::Method::Head => Ok(reqwest::Method::HEAD),
            wasi::http::types::Method::Options => Ok(reqwest::Method::OPTIONS),
            wasi::http::types::Method::Connect => Ok(reqwest::Method::CONNECT),
            wasi::http::types::Method::Trace => Ok(reqwest::Method::TRACE),
            wasi::http::types::Method::Patch => Ok(reqwest::Method::PATCH),
            wasi::http::types::Method::Other(m) => {
                match reqwest::Method::from_bytes(m.as_bytes()) {
                    Ok(m) => Ok(m),
                    Err(_) => return Err(ErrorCode::HTTPRequestMethodInvalid),
                }
            }
        }
    }
}

impl ToString for wasi::http::types::Scheme {
    fn to_string(&self) -> String {
        match self {
            wasi::http::types::Scheme::HTTP => "http".to_string(),
            wasi::http::types::Scheme::HTTPS => "https".to_string(),
            wasi::http::types::Scheme::Other(s) => s.clone(),
        }
    }
}
