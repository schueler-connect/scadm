use crate::{
  conn::Pipe,
  frame::{Frame, Reason},
  installation::Installation,
  status::DaemonStatus,
};

use super::{DaemonState, handle_err};

pub async fn start(dstate: &DaemonState, conn: &mut impl Pipe<Frame>) {
  let mut state = dstate.lock().await;

  let config = match state.config().await {
    Some(config) => Clone::clone(config),
    None => {
      conn
        .send_message(&Frame::Failed(Reason::MissingConfig))
        .ok();
      return;
    }
  };

  if let None = state.config().await {
    state.status = DaemonStatus::MissingConfig;
  }

  state.status = DaemonStatus::ServerStarting;

  let install = Installation::load().await;

  if !install.is_current(&config.server_version) {
    handle_err!(
      install.update(&config.server_version).await,
      |e| Reason::UpdateFailed(e),
      conn
    );
  }

  handle_err!(install.start().await, |e| Reason::StartingFailed(e), conn);

	state.status = DaemonStatus::ServerRunning;
}
