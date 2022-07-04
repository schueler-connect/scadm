use crate::{config::ConfigFile, constants::PID_FILE, status::DaemonStatus};
use std::{fs::remove_file, sync::Arc};
use tokio::sync::Mutex;

use std::ops::{Deref, DerefMut};

pub mod start;
pub use start::*;
pub mod stop;
pub use stop::*;

pub struct ExitGuard;

impl Drop for ExitGuard {
  fn drop(&mut self) {
    println!("Exiting scadmd");
    remove_file(PID_FILE).or::<()>(Ok(())).unwrap();
    println!("PID File Removed");
  }
}

pub struct InnerState {
  pub exit_guard: Option<ExitGuard>,
  pub status: DaemonStatus,
  /// This should be the _only_ source of truth for config information to the
  /// daemon. That way, consistency can be guaranteed because `DaemonState`
  /// should be held behind a mutex (see `State`)
  config: Option<ConfigFile>,
}

impl InnerState {
  async fn config(&mut self) -> Option<&ConfigFile> {
    if let None = self.config {
      self.config = ConfigFile::load().await.ok();
    }
    self.config.as_ref()
  }
}

pub type DaemonState = Arc<Mutex<InnerState>>;

pub struct State(Arc<Mutex<InnerState>>);

impl State {
  pub async fn new(e: ExitGuard) -> State {
    State(Arc::new(Mutex::new(InnerState {
      exit_guard: Some(e),
      status: DaemonStatus::Standby,
      config: ConfigFile::load().await.ok(),
    })))
  }
}

impl Deref for State {
  type Target = DaemonState;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for State {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

macro_rules! handle_err {
  ($e:expr, $h:expr, $conn:tt) => {
    match $e {
      Err(err) => {
        $conn.send_message(&Frame::Failed($h(err))).ok();
        return;
      }
      _ => {}
    };
  };
}

pub(in super::daemon) use handle_err;
