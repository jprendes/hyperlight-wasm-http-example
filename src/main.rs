extern crate alloc;

#[cfg(feature = "use-macro")]
mod bindings {
    hyperlight_component_macro::host_bindgen!();
}

// the bindings mod is like the above definition, but with an extra line that the
// macro is currently missing:
//   let mut rts = self.rt.lock().unwrap();
// without that line, the generated code doesn't compile
// See bindings.patch for the diff
#[cfg(not(feature = "use-macro"))]
mod bindings;

mod resource;
mod types;
mod worker;

use std::{convert::Infallible, net::SocketAddr, str::FromStr, sync::Arc};

use bindings::RootSandbox;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use hyperlight_host::sandbox::SandboxConfiguration;
use hyperlight_wasm::LoadedWasmSandbox;
use resource::{BlockOn, Resource};
use tokio::{net::TcpListener, sync::Mutex};
use types::{WasiImpl, http_incoming_body::IncomingBody, io_stream::Stream};
use worker::RUNTIME;

use crate::bindings::wasi::http::IncomingHandler;

fn main() {
    let builder = hyperlight_wasm::SandboxBuilder::new()
        .with_guest_heap_size(30 * 1024 * 1024)
        .with_guest_stack_size(1 * 1024 * 1024);

    // hyperlight wasm currently doesn't expose a way to set the host
    // function definition size, so we do it manually here with a
    // horrible hack to get a mutable reference to the config
    let config = builder.get_config() as *const _ as *mut SandboxConfiguration;
    let config = unsafe { config.as_mut().unwrap() };
    config.set_host_function_definition_size(20 * 1024);

    let mut sb = builder.build().unwrap();

    let state = types::WasiImpl::new();
    let rt = bindings::register_host_functions(&mut sb, state);

    let sb = sb.load_runtime().unwrap();
    let sb = sb.load_module("sample_wasi_http_rust.bin").unwrap();

    let sb = bindings::RootSandbox { sb, rt };
    let sb = Arc::new(Mutex::new(sb));

    RUNTIME.block_on(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let listener = TcpListener::bind(addr).await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();

            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(stream);

            let sb = sb.clone();

            RUNTIME.spawn(async move {
                // Finally, we bind the incoming connection to our `hello` service
                if let Err(err) = http1::Builder::new()
                    // `service_fn` converts our function in a `Service`
                    .serve_connection(
                        io,
                        service_fn(move |req: hyper::Request<hyper::body::Incoming>| {
                            hello(sb.clone(), req)
                        }),
                    )
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    });
}

async fn hello(
    sb: Arc<Mutex<RootSandbox<WasiImpl, LoadedWasmSandbox>>>,
    mut req: hyper::Request<hyper::body::Incoming>,
) -> Result<hyper::Response<Full<Bytes>>, Infallible> {
    let mut sb = sb.lock().await;
    let inst = bindings::root::component::RootExports::incoming_handler(&mut *sb);

    let body = req.body_mut();
    let mut full_body = Vec::new();
    while let Some(bytes) = body.frame().await {
        let Ok(bytes) = bytes else {
            let body = Full::new(Bytes::from("Error reading body"));
            let mut res = hyper::Response::new(body);
            *res.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(res);
        };
        let Ok(bytes) = bytes.into_data() else {
            // it's a trailers frame, skipt it
            // TODO: implement trailers handling
            continue;
        };
        full_body.extend_from_slice(&bytes);
    }

    let mut body_stream = Stream::new();
    let _ = body_stream.write(full_body);
    body_stream.close();

    let req = Resource::new(types::http_incoming_request::IncomingRequest {
        method: req.method().into(),
        path_with_query: Some(
            req.uri()
                .path_and_query()
                .map(|pq| pq.as_str().to_string())
                .unwrap_or_default(),
        ),
        scheme: Some(if req.uri().scheme_str() == Some("https") {
            bindings::wasi::http::types::Scheme::HTTPS
        } else {
            bindings::wasi::http::types::Scheme::HTTP
        }),
        authority: req.uri().authority().map(|a| a.as_str().to_string()),
        headers: Resource::new(req.headers().clone().into()),
        body: Resource::new(IncomingBody {
            stream: Resource::new(body_stream),
            trailers: Resource::default(),
            stream_taken: false,
        }),
        body_taken: false,
    });

    let outparam = Resource::default();

    inst.handle(req, outparam.clone());

    let Some(response) = outparam.write().block_on().response.take() else {
        let body = Full::new(Bytes::from("Error reading body"));
        let mut res = hyper::Response::new(body);
        *res.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;
        return Ok(res);
    };

    match response {
        Ok(response) => {
            let response = response.write().await;
            let status = response.status_code;
            let headers = response.headers.read().await.entries();
            let mut body = response.body.write_wait_until(|b| b.is_finished()).await;
            let body = body.read_all();
            let body_bytes = Bytes::from(body);

            let mut http_response = hyper::Response::new(Full::new(body_bytes));
            *http_response.status_mut() = hyper::StatusCode::from_u16(status)
                .unwrap_or(hyper::StatusCode::INTERNAL_SERVER_ERROR);
            let http_headers = http_response.headers_mut();
            for (k, v) in headers {
                let k = hyper::header::HeaderName::from_str(&k).unwrap();
                let v = hyper::header::HeaderValue::from_bytes(&v).unwrap();
                http_headers.append(k, v);
            }

            Ok(http_response)
        }
        Err(err) => {
            let body = Full::new(Bytes::from(format!("Error: {err:?}")));
            let mut res = hyper::Response::new(body);
            *res.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;
            Ok(res)
        }
    }
}

impl From<&hyper::Method> for bindings::wasi::http::types::Method {
    fn from(method: &hyper::Method) -> Self {
        match method.as_str() {
            "GET" => bindings::wasi::http::types::Method::Get,
            "POST" => bindings::wasi::http::types::Method::Post,
            "PUT" => bindings::wasi::http::types::Method::Put,
            "DELETE" => bindings::wasi::http::types::Method::Delete,
            "HEAD" => bindings::wasi::http::types::Method::Head,
            "OPTIONS" => bindings::wasi::http::types::Method::Options,
            "CONNECT" => bindings::wasi::http::types::Method::Connect,
            "TRACE" => bindings::wasi::http::types::Method::Trace,
            "PATCH" => bindings::wasi::http::types::Method::Patch,
            other => bindings::wasi::http::types::Method::Other(other.to_string()),
        }
    }
}
