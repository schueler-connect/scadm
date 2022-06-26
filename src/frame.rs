use rmp_serde::{decode::Deserializer, encode::Serializer};
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

pub enum Error {
  Incomplete,
  IO(io::Error),
  Unknown,
}

impl Into<super::error::Error> for Error {
  fn into(self) -> super::error::Error {
    match self {
      Error::Incomplete => super::error::Error::Parsing,
      Error::IO(e) => super::error::Error::IO(e),
      Error::Unknown => super::error::Error::Unknown,
    }
  }
}

impl From<rmp_serde::decode::Error> for Error {
  fn from(e: rmp_serde::decode::Error) -> Error {
    match e {
      rmp_serde::decode::Error::InvalidMarkerRead(e) => Error::IO(e),
      rmp_serde::decode::Error::InvalidDataRead(e) => Error::IO(e),
      _ => Error::Incomplete,
    }
  }
}

impl From<rmp_serde::encode::Error> for Error {
  fn from(_: rmp_serde::encode::Error) -> Error {
    Error::Unknown
  }
}

#[derive(Serialize, Deserialize)]
pub enum Frame {
  Stop,
  Stopped,
}

impl Frame {
  pub fn parse<T: Read>(buf: &mut T) -> Result<Frame, Error> {
    Ok(Frame::deserialize(&mut Deserializer::new(buf))?)
  }

  pub fn write<'e, W: Write>(&self, mut buf: W) -> Result<W, Error> {
    Frame::serialize(&self, &mut Serializer::new(&mut buf))?;
    Ok(buf)
  }
}
