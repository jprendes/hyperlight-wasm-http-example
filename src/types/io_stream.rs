use hyperlight_common::resource::BorrowedResourceGuard;

use crate::{
    bindings::wasi::{self, io::streams::StreamError},
    resource::{BlockOn, Resource},
};

use super::{
    WasiImpl,
    buffer::{Buffer, BufferClosed},
    io_poll::AnyPollable,
};

#[derive(Default)]
pub struct Stream {
    buffer: Buffer,
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

    pub fn check_write(&self) -> Result<u64, BufferClosed> {
        if self.buffer.is_closed() {
            return Err(BufferClosed);
        }
        Ok(4096)
    }

    pub fn write(&mut self, data: impl AsRef<[u8]>) -> Result<(), BufferClosed> {
        self.buffer.write(data)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), BufferClosed> {
        Ok(())
    }

    pub fn splice(&mut self, src: &mut Stream, len: usize) -> Result<usize, BufferClosed> {
        let n = self.check_write()? as usize;
        let n = n.min(len);
        let data = src.buffer.read(n)?;
        self.buffer.write(&data)?;
        Ok(data.len())
    }

    pub fn read(&mut self, len: usize) -> Result<Vec<u8>, BufferClosed> {
        self.buffer.read(len)
    }

    pub fn read_all(&mut self) -> Result<Vec<u8>, BufferClosed> {
        self.buffer.read_all()
    }

    pub fn readable(&self) -> bool {
        self.buffer.readable()
    }

    pub fn writable(&self) -> bool {
        self.buffer.writable()
    }

    pub fn close(&mut self) -> (usize, usize) {
        self.buffer.close()
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
        self_.write(&contents)?;
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
        self_.write(&vec![0; len as usize])?;
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
        let data = self_.read(len as usize)?;
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
        let data = self_.read(len as usize)?;
        Ok(data.len() as u64)
    }

    fn subscribe(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Resource<AnyPollable> {
        self_.poll(|b| b.readable())
    }
}

impl wasi::io::Streams<anyhow::Error, Resource<AnyPollable>> for WasiImpl {}
