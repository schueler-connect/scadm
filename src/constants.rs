// use std::fs::read_dir;

pub const DATA_PATH: &'static str = "/usr/local/scadm";
pub const PID_FILE: &'static str = "/tmp/scadmd.1.pid";
pub const SOCK_PATH: &'static str = "/tmp/scadmd.sock";

lazy_static::lazy_static! {
	// pub static ref SOCK_PATH: &'static str = {
	// 	if let Ok(_) = read_dir("/run") {
	// 		"/run/scadmd/scadmd.sock"
	// 	} else {
	// 		"/var/run/scadmd/scadmd.sock"
	// 	}
	// };
}
