use std::path::PathBuf;

// use std::fs::read_dir;
use home::home_dir;

pub const PID_FILE: &'static str = "/tmp/scadmd.1.pid";
pub const SOCK_PATH: &'static str = "/tmp/scadmd.sock";

lazy_static::lazy_static! {
  pub static ref DATA_PATH: PathBuf = {
    let mut p = home_dir().unwrap();
    p.push(".scadm");
    p
  };
  // pub static ref SOCK_PATH: &'static str = {
    // 	if let Ok(_) = read_dir("/run") {
  // 		"/run/scadmd/scadmd.sock"
  // 	} else {
  // 		"/var/run/scadmd/scadmd.sock"
  // 	}
  // };
}
