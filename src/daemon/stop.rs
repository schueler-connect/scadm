use crate::{
  conn::Pipe,
  frame::{Frame, Reason},
  installation::Installation,
  status::DaemonStatus,
};

use super::{handle_err, DaemonState};

pub async fn stop(dstate: &mut DaemonState, conn: &mut impl Pipe<Frame>) {
  let mut state = dstate.lock().await;

  state.status = DaemonStatus::ServerStopping;

  let install = Installation::load().await;

  handle_err!(install.stop().await, |e| Reason::StoppingFailed(e), conn);

  state.status = DaemonStatus::Standby;
}
