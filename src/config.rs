use semver::VersionReq;
use serde::{Deserialize, Serialize};
use serde_yaml::{from_str, to_string};
use tokio::{
  fs::{read_to_string, File},
  io::AsyncWriteExt,
};

use crate::{constants::CONFIG_PATH, Result};

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigFile {
	pub server_version: VersionReq
}

impl ConfigFile {
  pub async fn load() -> Result<ConfigFile> {
    Ok(from_str(&read_to_string(&*CONFIG_PATH).await?)?)
  }

  pub async fn flush(&self) -> Result<()> {
    Ok(
      File::create(&*CONFIG_PATH)
        .await?
        .write_all(to_string(self)?.as_bytes())
        .await?,
    )
  }
}
