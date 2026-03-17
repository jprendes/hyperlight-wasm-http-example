extern crate alloc;

mod sandbox_pool;
mod wasi_impl;

use wasi_impl::Resource;
use wasi_impl::bindings::wasi::cli::Run;
use wasi_impl::bindings::wasi::http::IncomingHandler;
use wasi_impl::types::{WasiImpl, http_incoming_body::IncomingBody, io_stream::Stream};
use wasi_impl::worker::RUNTIME;

use std::{convert::Infallible, net::SocketAddr, str::FromStr, sync::Arc};

use bytes::Bytes;
use clap::{Args, Parser, Subcommand};
use http_body_util::{BodyExt, Full};
use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use once_cell::sync::Lazy;
use sandbox_pool::SandboxPool;
use tokio::{net::TcpListener, sync::Mutex};

// Pool of sandboxes that are used at runtime to handle incoming HTTP requests.
static SANDBOX_POOL: Lazy<Arc<Mutex<SandboxPool>>> =
    Lazy::new(|| Arc::new(Mutex::new(SandboxPool::default())));

/// Common options shared between subcommands (modeled after wasmtime).
#[derive(Args, Debug, Clone)]
struct CliOptions {
    /// Not used, it needs `wasi:filesystem` implementation which is not
    /// possible at the moment because `wasi:filesystem/types` conflicts with
    /// `wasi:http/types`.
    /// This is here for compatibility with `wasmtime` cli
    #[arg(long = "dir", value_name = "HOST_DIR:GUEST_DIR")]
    dirs: Vec<String>,

    /// Pass an environment variable to the guest.
    /// Format: `KEY=VALUE`
    #[arg(long = "env", value_name = "KEY=VALUE")]
    envs: Vec<String>,

    /// Path to the AOT-compiled WASM component file
    wasm_file: String,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Run a WASM component (guest exports wasi:cli/run)
    Run {
        #[command(flatten)]
        options: CliOptions,
    },
    /// Serve an HTTP component (guest exports wasi:http/incoming-handler)
    Serve {
        #[command(flatten)]
        options: CliOptions,

        /// Socket address to listen on
        #[arg(long, default_value = "0.0.0.0:8888", value_name = "ADDR")]
        addr: String,
    },
}

#[derive(Parser, Debug)]
#[command(name = "hyperlight-wasm-debug")]
#[command(about = "Run a WASM component with Hyperlight")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Run { options } => {
            run_cli(options);
        }
        Command::Serve { options, addr } => {
            run_http(options, &addr);
        }
    }
}

/// Run the guest component via `wasi:cli/run`.
///
/// The guest manages its own I/O (TCP, UDP, stdio, etc.) through the WASI
/// imports it was compiled against.
fn run_cli(options: CliOptions) {
    println!("Running component...");
    RUNTIME.block_on(async {
        SANDBOX_POOL
            .lock()
            .await
            .from_options(options)
            .expect("failed to initialize sandbox pool");
        let mut sb = SANDBOX_POOL.lock().await.get_sandbox().expect("failed to get sandbox");
        tokio::task::block_in_place(|| {
            let inst = wasi_impl::bindings::root::component::RootExports::run(&mut sb);
            match inst.run() {
                Ok(()) => println!("Component exited successfully."),
                Err(()) => eprintln!("Component exited with an error."),
            }
        });
        SANDBOX_POOL.lock().await.return_sandbox(sb);
    });
}

/// Run an HTTP proxy server that forwards requests to the guest component's
/// `wasi:http/incoming-handler` export.
fn run_http(options: CliOptions, addr: &str) {
    let addr: SocketAddr = addr.parse().unwrap_or_else(|e| {
        eprintln!("Invalid address '{addr}': {e}");
        std::process::exit(1);
    });

    RUNTIME.block_on(async move {
        SANDBOX_POOL
            .lock()
            .await
            .from_options(options)
            .expect("failed to initialize sandbox pool");
        println!("Starting server on http://{addr}");

        let listener = TcpListener::bind(addr).await.unwrap();

        loop {
            let (stream, _) = listener.accept().await.unwrap();

            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(stream);

            RUNTIME.spawn(async move {
                // Finally, we bind the incoming connection to our `hello` service
                if let Err(err) = http1::Builder::new()
                    // `service_fn` converts our function in a `Service`
                    .serve_connection(
                        io,
                        service_fn(move |req: hyper::Request<hyper::body::Incoming>| hello(req)),
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
    mut req: hyper::Request<hyper::body::Incoming>,
) -> Result<hyper::Response<Full<Bytes>>, Infallible> {
    let mut sb = match SANDBOX_POOL.lock().await.get_sandbox() {
        Ok(sb) => sb,
        Err(err) => {
            eprintln!("Error getting sandbox: {err:?}");
            let body = Full::new(Bytes::from(format!("Error getting sandbox: {err}")));
            let mut res = hyper::Response::new(body);
            *res.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(res);
        }
    };
    let snapshot = sb.sb.snapshot().unwrap();
    let inst = wasi_impl::bindings::root::component::RootExports::incoming_handler(&mut sb);

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
    let _ = body_stream.write(&full_body);
    body_stream.close();

    let req = wasi_impl::types::http_incoming_request::IncomingRequest {
        method: req.method().into(),
        path_with_query: Some(
            req.uri()
                .path_and_query()
                .map(|pq| pq.as_str().to_string())
                .unwrap_or_default(),
        ),
        scheme: Some(if req.uri().scheme_str() == Some("https") {
            wasi_impl::bindings::wasi::http::types::Scheme::HTTPS
        } else {
            wasi_impl::bindings::wasi::http::types::Scheme::HTTP
        }),
        authority: req
            .uri()
            .authority()
            .map(|a| a.as_str())
            .or_else(|| req.headers().get("host").and_then(|h| h.to_str().ok()))
            .or_else(|| Some("dummy"))
            .map(|s| s.to_string()),
        headers: Resource::new(req.headers().clone().into()),
        body: Resource::new(IncomingBody {
            stream: Resource::new(body_stream),
            trailers: Resource::default(),
            stream_taken: false,
        }),
        body_taken: false,
    };
    let req = Resource::new(req);

    let outparam = Resource::default();

    tokio::task::block_in_place(|| {
        inst.handle(req, outparam.clone());
    });

    if let Err(err) = sb.sb.restore(&snapshot) {
        eprintln!("Error restoring snapshot: {err:?}");
        // Don't return the sandbox to the pool since restore failed.
        let body = Full::new(Bytes::from(format!("Error restoring snapshot: {err}")));
        let mut res = hyper::Response::new(body);
        *res.status_mut() = hyper::StatusCode::INTERNAL_SERVER_ERROR;
        return Ok(res);
    }

    // Return the sandbox to the pool immediately after the call, so it's available for the next
    // request while we wait for the guest to produce a response.
    SANDBOX_POOL.lock().await.return_sandbox(sb);

    let Some(response) = outparam.write().await.response.take() else {
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
            let body = body.read_all().await;
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

impl From<&hyper::Method> for wasi_impl::bindings::wasi::http::types::Method {
    fn from(method: &hyper::Method) -> Self {
        match method.as_str() {
            "GET" => wasi_impl::bindings::wasi::http::types::Method::Get,
            "POST" => wasi_impl::bindings::wasi::http::types::Method::Post,
            "PUT" => wasi_impl::bindings::wasi::http::types::Method::Put,
            "DELETE" => wasi_impl::bindings::wasi::http::types::Method::Delete,
            "HEAD" => wasi_impl::bindings::wasi::http::types::Method::Head,
            "OPTIONS" => wasi_impl::bindings::wasi::http::types::Method::Options,
            "CONNECT" => wasi_impl::bindings::wasi::http::types::Method::Connect,
            "TRACE" => wasi_impl::bindings::wasi::http::types::Method::Trace,
            "PATCH" => wasi_impl::bindings::wasi::http::types::Method::Patch,
            other => wasi_impl::bindings::wasi::http::types::Method::Other(other.to_string()),
        }
    }
}
