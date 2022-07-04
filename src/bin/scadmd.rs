use daemonize::Daemonize;
use scadm_core::conn::{Listener, Pipe, DummyConnection};
use scadm_core::daemon::{State, ExitGuard, start, DaemonState};
use std::fs::{create_dir_all, read_dir, File};
use std::path::{Path, PathBuf};
use std::process::exit;
use tokio::runtime::Builder;

use scadm_core::{
  conn::Connection,
  constants::{DATA_PATH, PID_FILE, SOCK_PATH},
  frame::Frame,
};

macro_rules! debug {
	($t:tt) => {
		if std::env::var("DEBUG").is_ok() {
			println!($t);
		}};
	($t:tt, $($e:expr),+) => {
		if std::env::var("DEBUG").is_ok() {
			println!($t, $($e),+);
		}}
}

pub(crate) use debug;

async fn handle_client(conn: &mut Connection, state: DaemonState) {
  println!("info: Received client connection");

  println!(
    "info: Constructed `Connection` around socket. Attempting to read \
frame"
  );

  while let Ok(frame) = conn.recv_message() {
    println!("info: Frame read from client");

    match frame {
      Frame::Stop() => {
        {
          let mut state = state.lock().await;
          drop(state.exit_guard.take());
        }
        println!("info: Sending response frame");
        conn.send_message(&Frame::Stopped()).unwrap();
        println!("info: Response frame sent");
        exit(0);
      }
      Frame::QueryStatus() => {
        let status = {
          let state = state.lock().await;
          state.status
        };
        conn.send_message(&Frame::Status(status)).unwrap();
      }
      _ => eprintln!(
        "notice: Ignoring unexpected message from \
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

  if let Err(_) = read_dir(&*DATA_PATH) {
    create_dir_all(&*DATA_PATH).unwrap();
  }

  let daemonize = Daemonize::new()
    .pid_file(PID_FILE)
    .chown_pid_file(true)
    // .group("daemon")
    .working_directory(&*DATA_PATH)
    .stdout(stdout)
    .privileged_action(|| {
      let folder_path = Path::new(&*SOCK_PATH).parent().unwrap();
      create_dir_all(folder_path).ok();
    })
    .stderr(stderr);

  daemonize.start().unwrap();

  println!("scadmd now active");

  Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async { tokio_main(e).await })
}

async fn tokio_main(e: ExitGuard) {
  let state = State::new(e).await;

  let listener = Listener::new(PathBuf::from(&*SOCK_PATH)).unwrap();

  let state_1 = state.clone();
  tokio::spawn(async move { start(&state_1, &mut DummyConnection).await; });

  loop {
    match listener.open() {
      Ok(mut conn) => {
        debug!("Incoming client connection to be handled");
        let state = state.clone();
        debug!("Cloned state");
        tokio::spawn(async move {
          debug!("Spawned tokio task");
          handle_client(&mut conn, state).await;
        });
      }
      Err(_) => eprintln!("notice: Connection failed"),
    }
  }
}
