use daemonize::Daemonize;
use parking_lot::Mutex;
use std::fs::{
  create_dir_all, read_dir, remove_file, set_permissions, File, Permissions,
};
use std::io::ErrorKind;
use std::os::unix::prelude::PermissionsExt;
use std::path::Path;
use std::process::exit;
use std::sync::Arc;
use tokio::{
  net::{UnixListener, UnixStream},
  runtime::Builder,
};

use scadm_core::{
  conn::Connection,
  constants::{DATA_PATH, PID_FILE, SOCK_PATH},
  frame::Frame,
};

struct ExitGuard;

impl Drop for ExitGuard {
  fn drop(&mut self) {
    println!("Exiting scadmd");
    remove_file(PID_FILE).or::<()>(Ok(())).unwrap();
    println!("PID File Removed");
  }
}

pub struct DaemonState {
  exit_guard: Option<ExitGuard>,
}

pub type State = Arc<Mutex<DaemonState>>;

async fn handle_client(sock: UnixStream, state: State) {
  let mut conn = Connection::from_stream(sock);

  while let Some(frame) = conn.read_frame().await.unwrap() {
    match frame {
      Frame::Stop => {
        {
          let mut state = state.lock();
          drop(state.exit_guard.take());
        }
        conn.write_frame(Frame::Stopped).await.unwrap();
        exit(0);
      }
      _ => eprintln!(
        "notice: Ignoring unexpected server response message from \
client"
      ),
    }
  }
}

fn main() {
  println!("starting scadmd; PIDFile: {}", &*PID_FILE);
  let e = ExitGuard;

  let stdout = File::create("/tmp/scadmd.out").unwrap();
  let stderr = File::create("/tmp/scadmd.err").unwrap();

  if let Ok(_) = File::open(PID_FILE) {
    indoc::eprintdoc! {
      "A PID file for scadmd already exists. This indicates scadmd may already
      be running or may have crashed. Please stop all running instances of
      scadmd, delete {file}, and try again.
      ",
      file = PID_FILE
    };
  }

  let daemonize = Daemonize::new()
    .pid_file(PID_FILE)
    .chown_pid_file(true)
    // .group("daemon")
    .working_directory(DATA_PATH)
    .stdout(stdout)
    .privileged_action(|| {
      let folder_path = Path::new(&*SOCK_PATH).parent().unwrap();
      create_dir_all(folder_path).unwrap();
      set_permissions(folder_path, Permissions::from_mode(0o777))
        .or_else(|e| {
          if e.kind() == ErrorKind::PermissionDenied {
						eprintln!("Bitte starten sie das daemon mit root-berechtigungen");
						exit(1);
					} else {
            Err(e)
          }
        })
        .unwrap();
    })
    .stderr(stderr);

  daemonize.start().unwrap();

  println!("scadmd now active");

  Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      tokio_main(e).await;
    })
}

async fn tokio_main(e: ExitGuard) {
  if let Err(_) = read_dir(DATA_PATH) {
    create_dir_all(DATA_PATH).unwrap();
  }

	remove_file(&*SOCK_PATH).unwrap();
  let listener = UnixListener::bind(&*SOCK_PATH).unwrap();

  let state = Arc::new(Mutex::new(DaemonState {
    exit_guard: Some(e),
  }));

  loop {
    match listener.accept().await {
      Ok((sock, _addr)) => {
        let state = state.clone();
        tokio::spawn(async move {
          handle_client(sock, state).await;
        });
      }
      Err(_) => eprintln!("notice: Connection failed"),
    }
  }
}
