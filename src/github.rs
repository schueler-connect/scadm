use async_compression::futures::bufread::GzipDecoder;
use bytes::Bytes;
use futures::{
  stream::MapErr,
  stream::{IntoAsyncRead, TryStreamExt},
  Stream,
};
use reqwest::{get, Error};
use serde::{Deserialize, Serialize};
use serde_yaml::from_str;

use crate::Result;

#[derive(Serialize, Deserialize)]
pub struct Object {
  pub sha: String,
  pub r#type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ref {
  pub r#ref: String,
  pub node_id: String,
  pub url: String,
  pub object: Object,
}

pub async fn list_tags(owner: &str, repo: &str) -> Result<Vec<Ref>> {
  Ok(from_str(
    &get(format!(
      "https://api.github.com/repos/{}/{}/git/refs/tags",
      owner, repo
    ))
    .await?
    .text()
    .await?,
  )?)
}

pub async fn get_tarball(
  owner: &str,
  repo: &str,
  r#ref: &str,
) -> Result<
  GzipDecoder<
    IntoAsyncRead<
      MapErr<
        impl Stream<Item = std::result::Result<Bytes, Error>>,
        impl FnMut(Error) -> futures::io::Error,
      >,
    >,
  >,
> {
  let stream = get(format!(
    "https://api.github.com/repos/{}/{}/tarball/{}",
    owner, repo, r#ref
  ))
  .await?
  .bytes_stream()
  .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
  .into_async_read();

  let tar = GzipDecoder::new(stream);

  Ok(tar)
}
