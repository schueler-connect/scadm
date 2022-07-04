pub mod conn;
pub mod error;
pub mod frame;
pub use error::Result;
pub mod commands;
pub mod constants;
pub mod status;
pub mod daemon;
pub mod config;
pub mod installation;
pub mod github;

macro_rules! debug {
	($t:tt) => {
		if std::env::var("DEBUG").is_ok() {
			println!($t);
		}
	};
	($t:tt, $($e:expr),+) => {
		if std::env::var("DEBUG").is_ok() {
			println!($t, $($e),+);
		}}
}

pub(crate) use debug;
