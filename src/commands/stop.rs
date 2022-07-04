use crate::{
  conn::{Connection, Pipe},
  constants::{PID_FILE, SOCK_PATH},
  error::Error,
  frame::Frame,
  Result,
};
use nix::errno::Errno;
use std::{
  fs::{read_to_string, remove_file},
  path::Path,
  process::exit,
  time::Duration,
};
use tokio::time::timeout;

async fn end() -> Result<()> {
  let mut conn = Connection::open(Path::new(&*SOCK_PATH))?;

  conn.send_message(&Frame::Stop())?;

  if let Ok(frame) = conn.recv_message() {
    match frame {
      Frame::Stopped() => {
        println!("Erfolgreich beendet");
      }
      _ => return Err(Error::WithMessage("Unerwartete Nachricht".to_owned())),
    }
  } else {
    return Err(Error::WithMessage(
      "Fehler bei der normalen beendigung".to_owned(),
    ));
  }

  Ok::<(), Error>(())
}

async fn kill() -> Result<()> {
  let pid = read_to_string(Path::new(PID_FILE))
    .or_else(
      #[allow(unreachable_code)]
      |_| {
        eprintln!("PID-datei konnte nicht eingelesen werden");
        exit(1);
        Err(())
      },
    )
    .unwrap()
    .parse()?;

  nix::sys::signal::kill(
    nix::unistd::Pid::from_raw(pid),
    nix::sys::signal::SIGKILL,
  )
  .or_else(|e| {
    if e == Errno::ESRCH {
      println!("info: Prozess läuft nicht");
      Ok(())
    } else {
      Err(e)
    }
  })
  .unwrap();
  println!("Beendigung erzwungen");

  Ok(())
}

pub async fn stop(force: bool) {
  println!("Wird beendet");

  let pid_file_path = Path::new(PID_FILE);

  if pid_file_path.exists() {
    if force {
      println!("info: Beendigung wird im notfall erzwungen");
      let to = timeout(Duration::from_secs(60), async move {
        #[allow(unused_must_use)]
        if let Err(_) = end().await {
          kill().await;
        }
      })
      .await;

      match to {
        #[allow(unused_must_use)]
        Err(_) => {
          kill().await;
        }
        _ => {}
      };
    } else {
      end().await.unwrap();
    }
  } else {
    println!("scadmd läuft nicht");
  }

  if pid_file_path.exists() {
    match remove_file(pid_file_path) {
      Ok(_) => println!("PID-Datei entfernt"),
      Err(_) => println!("warn: PID-Datei konnte nicht entfernt werden"),
    };
  }
}
