use std::{
  fs::File,
  io::{ErrorKind, Write},
  path::PathBuf,
  process::Stdio,
};

use crate::{
  constants::DATA_PATH,
  error::Error,
  github::{get_tarball, list_tags},
  Result,
};
use lazy_static::lazy_static;
use semver::{BuildMetadata, Prerelease, Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_yaml::{from_str, to_string};
use tokio::{
  fs::{create_dir_all, read_dir, read_to_string, remove_dir_all},
  process::Command,
};

lazy_static! {
  static ref INSTALL_PATH_A: PathBuf = {
    let mut p = DATA_PATH.clone();
    p.push("server-a");
    p
  };
  static ref INSTALL_PATH_B: PathBuf = {
    let mut p = DATA_PATH.clone();
    p.push("server-b");
    p
  };
  //
  static ref MANIFEST_PATH: PathBuf = {
    let mut p = DATA_PATH.clone();
    p.push("servermanifest.yaml");
    p
  };
  //
  static ref SHARED_VOLUMES: PathBuf = {
    let mut p = DATA_PATH.clone();
    p.push("shared/volumes");
    p
  };
  static ref VOLUMES_MONGODB: PathBuf = {
    let mut p = SHARED_VOLUMES.clone();
    p.push("mongodb");
    p
  };
}

pub enum UpdateStatus {
  UpToDate,
  PatchBehind,
  MinorBehind,
  MajorBehind,
  CriticalBehind,
  NotInstalled,
}

#[derive(Serialize, Deserialize)]
pub struct ServerDistribution {
  pub owner: String,
  pub repo: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServerManifest {
  pub version: Version,
  pub distribution: ServerDistribution,
}

impl Default for ServerManifest {
  fn default() -> Self {
    ServerManifest {
      version: Version {
        major: 0,
        minor: 0,
        patch: 0,
        pre: Prerelease::EMPTY,
        build: BuildMetadata::EMPTY,
      },
      distribution: ServerDistribution {
        owner: String::from("schueler-connect"),
        repo: String::from("backend"),
      },
    }
  }
}

pub struct Installation {
  pub installed: bool,
  manifest: ServerManifest,
}

impl Installation {
  pub async fn load() -> Installation {
    let is_installed = Self::get_active_install().await.is_ok();

    Installation {
      installed: is_installed,
      manifest: from_str(
        &read_to_string(&*MANIFEST_PATH)
          .await
          .or_else(|e| {
            if let ErrorKind::NotFound = e.kind() {
              let mut m = File::create(&*MANIFEST_PATH)?;

              m.write_all(
                to_string(&ServerManifest::default()).unwrap().as_bytes(),
              )
              .unwrap();
            }
            Err(e)
          })
          .unwrap(),
      )
      .unwrap(),
    }
  }

  pub async fn get_active_install() -> Result<PathBuf> {
    if read_dir(&*INSTALL_PATH_A).await.is_ok() {
      Ok(INSTALL_PATH_A.clone())
    } else if read_dir(&*INSTALL_PATH_B).await.is_ok() {
      Ok(INSTALL_PATH_B.clone())
    } else {
      Err(Error::WithMessage(
        "Neither installation available".to_string(),
      ))
    }
  }

  pub async fn get_inactive_install() -> Result<PathBuf> {
    if read_dir(&*INSTALL_PATH_A).await.is_err() {
      Ok(INSTALL_PATH_A.clone())
    } else if read_dir(&*INSTALL_PATH_B).await.is_err() {
      Ok(INSTALL_PATH_B.clone())
    } else {
      Err(Error::WithMessage("Both installations in use".to_string()))
    }
  }

  pub fn is_current(&self, for_target: &VersionReq) -> bool {
    for_target.matches(&self.manifest.version)
  }

  pub async fn update(&self, req: &VersionReq) -> Result<()> {
    if self.is_current(&req) {
      return Ok(());
    }

    let mut all_tags: Vec<String> = {
      let tags = list_tags(
        &self.manifest.distribution.owner,
        &self.manifest.distribution.repo,
      )
      .await?;
      tags
        .iter()
        // Split at 11 to strip out /refs/tags/v from e.g. /refs/tags/v1.2.3
        .map(|r| r.r#ref.split_at(11).1.to_owned())
        .collect()
    };

    all_tags.sort();

    let target_version = all_tags.iter().find(|&tag| {
      let tag_version = Version::parse(&tag);

      if let Ok(tag_version) = tag_version {
        req.matches(&tag_version)
      } else {
        false
      }
    });

    let target_version = match target_version {
      Some(v) => v,
      None => {
        return Err(Error::WithMessage(
          "Couldn't locate a matching version tag".to_string(),
        ));
      }
    };

    let tarball = get_tarball(
      &self.manifest.distribution.owner,
      &self.manifest.distribution.repo,
      &format!("tags/v{}", &target_version),
    )
    .await?;

    let archive = async_tar::Archive::new(tarball);

    let target_dir = Self::get_inactive_install().await?;

    create_dir_all(&target_dir).await?;

    archive.unpack(&target_dir).await?;

    self.switch_installations(&target_dir).await?;

    Ok(())
  }

  pub async fn switch_installations(&self, to: &PathBuf) -> Result<()> {
    if Self::get_inactive_install().await.is_ok() {
      return Err(Error::WithMessage(
        "Only one installations appears to be populated".to_string(),
      ));
    }

    self.stop().await?;

    remove_dir_all(if to == &*INSTALL_PATH_A {
      &*INSTALL_PATH_B
    } else {
      &*INSTALL_PATH_A
    })
    .await?;

    self.start().await?;

    Ok(())
  }

  pub async fn start(&self) -> Result<()> {
    // Make sure shared volumes exist
    create_dir_all(&*VOLUMES_MONGODB).await?;

    // TODO: Build docker-compose.selfhosting.yaml

    Command::new("npm")
      .current_dir(Self::get_active_install().await?)
      .arg("run")
      .arg("selfhost")
      .stdin(Stdio::null())
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .spawn()
      .or_else(|_| {
        Err(Error::WithMessage(
          "Couldn't run `npm run selfhost`".to_string(),
        ))
      })?
      .wait()
      .await
      .or_else(|_| {
        Err(Error::WithMessage(
          "Failed to start server for selfhosting".to_string(),
        ))
      })?;

    Ok(())
  }

  pub async fn stop(&self) -> Result<()> {
    Command::new("npm")
      .current_dir(Self::get_active_install().await?)
      .arg("run")
      .arg("teardown")
      .stdin(Stdio::null())
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .spawn()
      .or_else(|_| {
        Err(Error::WithMessage(
          "Couldn't run `npm run teardown`".to_string(),
        ))
      })?
      .wait()
      .await
      .or_else(|_| {
        Err(Error::WithMessage(
          "Failed to tear down server".to_string(),
        ))
      })?;

    Ok(())
  }
}
