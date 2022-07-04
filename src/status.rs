use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
pub enum DaemonStatus {
  /// Server is stopped, but the daemon detects no issues
  Standby,
  /// User configuration file is missing
  MissingConfig,
  /// User configuration file is invalid
  InvalidConfig,
  /// Server is in the process of stopping
  ServerStopping,
  /// Server is in the process of restarting
  ServerRestarting,
  /// Server is in the process of starting
  ServerStarting,
  /// Server is currently running
  ServerRunning,
}
