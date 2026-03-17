use hyperlight_common::resource::BorrowedResourceGuard;

use crate::wasi_impl::{
    bindings::wasi::{self, io::streams::StreamError},
    resource::{BlockOn, Resource},
};

use super::{
    WasiImpl,
    buffer::{Buffer, BufferClosed},
    io_poll::AnyPollable,
};

use std::sync::Arc;

/// A WASI stream that is either backed by an in-memory buffer or directly
/// by one half of a TCP connection (read or write).
pub enum Stream {
    Buffer(Buffer),
    TcpRead {
        socket: Arc<tokio::net::TcpStream>,
        closed: bool,
    },
    TcpWrite {
        socket: Arc<tokio::net::TcpStream>,
        closed: bool,
    },
}

impl Default for Stream {
    fn default() -> Self {
        Self::Buffer(Buffer::default())
    }
}

impl<E> From<BufferClosed> for wasi::io::streams::StreamError<E> {
    fn from(_: BufferClosed) -> Self {
        wasi::io::streams::StreamError::Closed
    }
}

impl Stream {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tcp_read(socket: Arc<tokio::net::TcpStream>) -> Self {
        Self::TcpRead {
            socket,
            closed: false,
        }
    }

    pub fn tcp_write(socket: Arc<tokio::net::TcpStream>) -> Self {
        Self::TcpWrite {
            socket,
            closed: false,
        }
    }

    pub fn check_write(&self) -> Result<u64, BufferClosed> {
        match self {
            Self::Buffer(buf) => {
                if buf.is_closed() {
                    return Err(BufferClosed);
                }
                Ok(4096)
            }
            Self::TcpWrite { closed: true, .. } => Err(BufferClosed),
            Self::TcpWrite { .. } => Ok(4096),
            Self::TcpRead { .. } => Err(BufferClosed), // can't write to a read stream
        }
    }

    pub fn write(&mut self, data: impl AsRef<[u8]>) -> Result<(), BufferClosed> {
        self.blocking_write(data)
    }

    /// Blocking write: loops until all data is written to the TCP socket.
    pub fn blocking_write(&mut self, data: impl AsRef<[u8]>) -> Result<(), BufferClosed> {
        match self {
            Self::Buffer(buf) => buf.write(data),
            Self::TcpWrite {
                socket,
                closed: false,
            } => {
                let data = data.as_ref();
                let mut offset = 0;
                while offset < data.len() {
                    match socket.try_write(&data[offset..]) {
                        Ok(n) => offset += n,
                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            let sock = socket.clone();
                            let _ = async { sock.writable().await }.block_on();
                        }
                        Err(_) => {
                            *self = Self::TcpWrite {
                                socket: socket.clone(),
                                closed: true,
                            };
                            return Err(BufferClosed);
                        }
                    }
                }
                Ok(())
            }
            _ => Err(BufferClosed),
        }
    }

    pub fn flush(&mut self) -> Result<(), BufferClosed> {
        Ok(())
    }

    pub fn splice(&mut self, src: &mut Stream, len: usize) -> Result<usize, BufferClosed> {
        let n = self.check_write()? as usize;
        let n = n.min(len);
        let data = src.read(n)?;
        self.write(&data)?;
        Ok(data.len())
    }

    pub fn read(&mut self, len: usize) -> Result<Vec<u8>, BufferClosed> {
        match self {
            Self::Buffer(buf) => buf.read(len),
            Self::TcpRead {
                socket,
                closed: false,
            } => {
                let mut buf = vec![0u8; len];
                match socket.try_read(&mut buf) {
                    Ok(0) => {
                        *self = Self::TcpRead {
                            socket: socket.clone(),
                            closed: true,
                        };
                        Err(BufferClosed)
                    }
                    Ok(n) => {
                        buf.truncate(n);
                        Ok(buf)
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(vec![]),
                    Err(_) => {
                        *self = Self::TcpRead {
                            socket: socket.clone(),
                            closed: true,
                        };
                        Err(BufferClosed)
                    }
                }
            }
            _ => Err(BufferClosed),
        }
    }

    pub fn read_all(&mut self) -> Result<Vec<u8>, BufferClosed> {
        match self {
            Self::Buffer(buf) => buf.read_all(),
            // For TCP, read whatever is available (up to 64k).
            _ => self.read(65536),
        }
    }

    /// Blocking read: waits until data is available on TCP sockets.
    pub fn blocking_read(&mut self, len: usize) -> Result<Vec<u8>, BufferClosed> {
        match self {
            Self::TcpRead {
                socket,
                closed: false,
            } => {
                let sock = socket.clone();
                let _ = async { sock.readable().await }.block_on();
                self.read(len)
            }
            // For buffer-backed streams, read is already handled by the
            // write_wait_until(readable) pattern at the call site.
            _ => self.read(len),
        }
    }

    pub fn readable(&self) -> bool {
        match self {
            Self::Buffer(buf) => buf.readable(),
            Self::TcpRead { closed: true, .. } => true, // closed is "readable" (triggers EOF)
            Self::TcpRead { .. } => true, // always worth trying a non-blocking read
            Self::TcpWrite { .. } => false,
        }
    }

    pub fn writable(&self) -> bool {
        match self {
            Self::Buffer(buf) => buf.writable(),
            Self::TcpWrite { closed: true, .. } => false,
            Self::TcpWrite { .. } => true,
            Self::TcpRead { .. } => false,
        }
    }

    pub fn close(&mut self) -> (usize, usize) {
        match self {
            Self::Buffer(buf) => buf.close(),
            Self::TcpRead { closed, .. } | Self::TcpWrite { closed, .. } => {
                *closed = true;
                (0, 0)
            }
        }
    }
}

impl wasi::io::streams::OutputStream<anyhow::Error, Resource<Stream>, Resource<AnyPollable>>
    for WasiImpl
{
    type T = Resource<Stream>;

    fn check_write(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<u64, StreamError<anyhow::Error>> {
        let self_ = self_.read().block_on();
        let n = self_.check_write()?;
        Ok(n)
    }

    fn write(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        contents: Vec<u8>,
    ) -> Result<(), StreamError<anyhow::Error>> {
        let mut self_ = self_.write().block_on();
        self_.write(&contents)?;
        Ok(())
    }

    fn blocking_write_and_flush(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        contents: Vec<u8>,
    ) -> Result<(), StreamError<anyhow::Error>> {
        let mut self_ = self_.write_wait_until(Stream::writable).block_on();
        self_.blocking_write(&contents)?;
        self_.flush()?;
        Ok(())
    }

    fn flush(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<(), StreamError<anyhow::Error>> {
        let mut self_ = self_.write().block_on();
        self_.flush()?;
        Ok(())
    }

    fn blocking_flush(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<(), StreamError<anyhow::Error>> {
        let mut self_ = self_.write().block_on();
        self_.flush()?;
        Ok(())
    }

    fn write_zeroes(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        len: u64,
    ) -> Result<(), StreamError<anyhow::Error>> {
        let mut self_ = self_.write().block_on();
        self_.write(&vec![0; len as usize])?;
        Ok(())
    }

    fn blocking_write_zeroes_and_flush(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        len: u64,
    ) -> Result<(), StreamError<anyhow::Error>> {
        let mut self_ = self_.write().block_on();
        self_.blocking_write(&vec![0; len as usize])?;
        self_.flush()?;
        Ok(())
    }

    fn splice(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        src: BorrowedResourceGuard<Resource<Stream>>,
        len: u64,
    ) -> Result<u64, StreamError<anyhow::Error>> {
        let mut self_ = self_.write().block_on();
        let mut src = src.write().block_on();
        let n = self_.splice(&mut src, len as _)?;
        Ok(n as u64)
    }

    fn blocking_splice(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        src: BorrowedResourceGuard<Resource<Stream>>,
        len: u64,
    ) -> Result<u64, StreamError<anyhow::Error>> {
        let mut self_ = self_.write_wait_until(Stream::writable).block_on();
        let mut src = src.write_wait_until(Stream::readable).block_on();
        let n = self_.splice(&mut src, len as _)?;
        Ok(n as u64)
    }

    fn subscribe(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Resource<AnyPollable> {
        self_.poll(|b| b.writable())
    }
}

impl wasi::io::streams::InputStream<anyhow::Error, Resource<AnyPollable>> for WasiImpl {
    type T = Resource<Stream>;

    fn read(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        len: u64,
    ) -> Result<Vec<u8>, StreamError<anyhow::Error>> {
        let mut self_ = self_.write().block_on();
        let data = self_.read(len as usize)?;
        Ok(data)
    }

    fn blocking_read(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        len: u64,
    ) -> Result<Vec<u8>, StreamError<anyhow::Error>> {
        let mut self_ = self_.write_wait_until(Stream::readable).block_on();
        let data = self_.blocking_read(len as usize)?;
        Ok(data)
    }

    fn skip(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        len: u64,
    ) -> Result<u64, StreamError<anyhow::Error>> {
        let mut self_ = self_.write().block_on();
        let data = self_.read(len as usize)?;
        Ok(data.len() as u64)
    }

    fn blocking_skip(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        len: u64,
    ) -> Result<u64, StreamError<anyhow::Error>> {
        let mut self_ = self_.write_wait_until(Stream::readable).block_on();
        let data = self_.blocking_read(len as usize)?;
        Ok(data.len() as u64)
    }

    fn subscribe(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Resource<AnyPollable> {
        self_.poll(|b| b.readable())
    }
}

impl wasi::io::Streams<anyhow::Error, Resource<AnyPollable>> for WasiImpl {}
