use std::{collections::VecDeque, mem};

#[derive(Default)]
pub struct Buffer {
    buffer: VecDeque<u8>,
    closed: bool,
    total_written: usize,
    total_read: usize,
}

pub struct BufferClosed;

impl Buffer {
    pub fn write(&mut self, data: impl AsRef<[u8]>) -> Result<(), BufferClosed> {
        if self.closed {
            return Err(BufferClosed);
        }
        self.buffer.extend(data.as_ref());
        self.total_written += data.as_ref().len();
        Ok(())
    }

    pub fn writable(&self) -> bool {
        true
    }

    pub fn read(&mut self, n: usize) -> Result<Vec<u8>, BufferClosed> {
        if self.buffer.is_empty() && self.closed {
            return Err(BufferClosed);
        }
        let n = n.min(self.buffer.len());
        let mut tail = self.buffer.split_off(n);
        mem::swap(&mut self.buffer, &mut tail);
        self.total_read += n;
        Ok(tail.into())
    }

    pub fn readable(&self) -> bool {
        self.closed || !self.buffer.is_empty()
    }

    pub fn read_all(&mut self) -> Result<Vec<u8>, BufferClosed> {
        if self.buffer.is_empty() && self.closed {
            return Err(BufferClosed);
        }
        let mut tail = VecDeque::new();
        mem::swap(&mut self.buffer, &mut tail);
        let n = tail.len();
        self.total_read += n;
        Ok(tail.into())
    }

    pub fn close(&mut self) -> (usize, usize) {
        self.closed = true;
        (self.total_read, self.total_written)
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }
}
