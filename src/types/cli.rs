use std::sync::LazyLock;

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _, stderr, stdin, stdout};

use crate::{bindings::wasi, resource::Resource, worker::RUNTIME};

use super::{WasiImpl, io_stream::Stream};

static STDIN: LazyLock<Resource<Stream>> = LazyLock::new(|| {
    let stream = Resource::new(Stream::default());
    let stream_clone = stream.clone();
    RUNTIME.spawn(async move {
        loop {
            let buf = &mut [0u8; 1024];
            let n = stdin().read(buf).await.unwrap();
            if n == 0 {
                // EOF
                break;
            }
            let buf = &buf[..n];
            let mut stream = stream_clone.write().await;
            let _ = stream.write(buf);
        }
    });
    stream
});

static STDOUT: LazyLock<Resource<Stream>> = LazyLock::new(|| {
    let stream = Resource::new(Stream::default());
    let stream_clone = stream.clone();
    RUNTIME.spawn(async move {
        loop {
            let mut stream = stream_clone.write_wait_until(|s| s.readable()).await;
            let Ok(data) = stream.read_all() else {
                // stream closed
                break;
            };
            let _ = stdout().write_all(&data).await;
        }
    });
    stream
});

static STDERR: LazyLock<Resource<Stream>> = LazyLock::new(|| {
    let stream = Resource::new(Stream::default());
    let stream_clone = stream.clone();
    RUNTIME.spawn(async move {
        loop {
            let mut stream = stream_clone.write_wait_until(|s| s.readable()).await;
            let Ok(data) = stream.read_all() else {
                // stream closed
                break;
            };
            let _ = stderr().write_all(&data).await;
        }
    });
    stream
});

impl wasi::cli::Stdin<Resource<Stream>> for WasiImpl {
    fn get_stdin(&mut self) -> Resource<Stream> {
        STDIN.clone()
    }
}

impl wasi::cli::Stdout<Resource<Stream>> for WasiImpl {
    fn get_stdout(&mut self) -> Resource<Stream> {
        STDOUT.clone()
    }
}

impl wasi::cli::Stderr<Resource<Stream>> for WasiImpl {
    fn get_stderr(&mut self) -> Resource<Stream> {
        STDERR.clone()
    }
}
