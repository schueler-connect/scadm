use super::{
  error::Result,
  frame::{self, Frame},
};
use bytes::BytesMut;
use std::{io::Cursor, path::Path};
use tokio::{
  io::{AsyncReadExt, AsyncWriteExt, BufWriter},
  net::UnixStream,
};

pub struct Connection {
  stream: BufWriter<UnixStream>,
  buffer: BytesMut,
}

impl Connection {
  pub fn from_stream(stream: UnixStream) -> Self {
    Connection {
      stream: BufWriter::new(stream),
      buffer: BytesMut::with_capacity(8 * 1024),
    }
  }

  pub async fn connect(path: &Path) -> Result<Self> {
    Ok(Connection {
      stream: BufWriter::new(UnixStream::connect(path).await?),
      buffer: BytesMut::with_capacity(8 * 1024),
    })
  }

  pub async fn read_frame(&mut self) -> crate::Result<Option<Frame>> {
    loop {
      if let Some(frame) = self.parse_frame()? {
        return Ok(Some(frame));
      }

      if 0 == self.stream.read_buf(&mut self.buffer).await? {
        if self.buffer.is_empty() {
          return Ok(None);
        } else {
          return Err("connection reset by peer".into());
        }
      }
    }
  }

  pub async fn write_frame(&mut self, frame: Frame) -> Result<()> {
    let mut buf: Vec<u8> = Vec::with_capacity(8 * 1024);
    let buf = frame
      .write(&mut buf)
      .or_else(|e: frame::Error| -> Result<&mut Vec<u8>> { Err(e.into()) })?;
    self.stream.write_all(&buf[..]).await?;
    Ok(())
  }

  fn parse_frame(&mut self) -> crate::Result<Option<Frame>> {
    use frame::Error::Incomplete;

    let mut buf = Cursor::new(&self.buffer[..]);

    match Frame::parse(&mut buf) {
      Ok(frame) => Ok(Some(frame)),
      Err(Incomplete) => Ok(None),
      Err(e) => Err(e.into()),
    }
  }
}
