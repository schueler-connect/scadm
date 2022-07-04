use std::{fmt, num::ParseIntError};

use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug)]
pub enum Error {
  WithMessage(String),
  IO(tokio::io::Error),
  HTTP(reqwest::Error),
  Parsing,
  Unknown,
  Serialized(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<&str> for Error {
  fn from(e: &str) -> Error {
    Error::WithMessage(e.to_owned())
  }
}

impl From<tokio::io::Error> for Error {
  fn from(e: tokio::io::Error) -> Error {
    Error::IO(e)
  }
}

impl From<ParseIntError> for Error {
  fn from(_: ParseIntError) -> Error {
    Error::Parsing
  }
}

impl From<serde_yaml::Error> for Error {
  fn from(e: serde_yaml::Error) -> Error {
    Error::WithMessage(e.to_string())
  }
}

impl From<reqwest::Error> for Error {
  fn from(e: reqwest::Error) -> Error {
    Error::HTTP(e)
  }
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&format!("{:#?}", self))
  }
}

impl<'de> Deserialize<'de> for Error {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct Vis;

    impl<'de> Visitor<'de> for Vis {
      type Value = Error;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("error messsage")
      }

      fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(Error::Serialized(v.to_string()))
      }
    }

    deserializer.deserialize_str(Vis)
  }
}
