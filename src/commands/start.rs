use crate::constants::PID_FILE;
use std::{path::Path, process::{Command, exit}};

use super::stop::stop;

pub async fn start(force: bool) {
  if Path::new(PID_FILE).exists() {
    if force {
      stop(true).await;
    } else {
			eprintln!("Eine PID-Datei für scadmd besteht bereits. Dies bedeutet, \
dass scadmd entweder bereits läuft, oder abgestürzt ist. Wenn sie sich sicher \
sind, dass scadmd nicht läuft, können sie den neustart mit --force erzwingen");
			exit(1);
		}
  }

  #[cfg(debug_assertions)]
  Command::new("cargo")
    .args(["run", "--bin", "scadmd", "--"])
		.env("RUST_BACKTRACE", "full")
    .spawn()
    .expect("Starten fehlgeschlagen");

  #[cfg(not(debug_assertions))]
  Command::new("scadmd")
    .spawn()
    .expect("Starten fehlgeschlagen");

	println!("Erfolgreich gestartet");
}
