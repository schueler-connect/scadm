use serde::{Deserialize, Serialize};

use crate::{status::DaemonStatus, error::Error};

#[derive(Serialize, Deserialize)]
pub enum Reason {
	MissingConfig,
	UpdateFailed(Error),
	StartingFailed(Error),
	StoppingFailed(Error)
}

#[derive(Serialize, Deserialize)]
pub enum Frame {
  Dummy(),
  Stop(),
  Stopped(),
  QueryStatus(),
  Status(DaemonStatus),
	Failed(Reason)
}

impl Default for Frame {
  fn default() -> Self {
    Frame::Dummy()
  }
}
