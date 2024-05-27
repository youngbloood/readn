use anyhow::Result;
use bytes::BytesMut;
use std::{io::Read, pin::Pin};
use tokio::io::AsyncReadExt;

/// ReadN is a struct that support read length n from the synchronous reader.
pub struct ReadN<'a> {
    reader: &'a mut dyn Read,
    buf: BytesMut,
}

impl<'a> ReadN<'a> {
    pub fn new(reader: &'a mut dyn Read) -> Self {
        ReadN {
            reader,
            buf: BytesMut::new(),
        }
    }

    pub fn read(&mut self, n: usize) -> Result<Vec<u8>> {
        self.buf.resize(n, 0);
        self.reader.read_exact(&mut self.buf)?;
        Ok(self.buf.to_vec())
    }
}

/// ReadN is a struct that support read length n from the asynchronous reader.
pub struct AsyncReadN<'a, T>
where
    T: AsyncReadExt + Unpin,
{
    reader: Pin<Box<&'a mut T>>,
    buf: BytesMut,
}

impl<'a, T> AsyncReadN<'a, T>
where
    T: AsyncReadExt + Unpin,
{
    pub fn new(reader: &'a mut T) -> Self {
        let bp = Box::pin(reader);
        AsyncReadN {
            reader: bp,
            buf: BytesMut::new(),
        }
    }

    pub async fn read(&mut self, n: usize) -> Result<Vec<u8>> {
        self.buf.resize(n, 0);
        self.reader.read_exact(&mut self.buf).await?;
        Ok(self.buf.to_vec())
    }
}
