// Adapted from https://gist.github.com/rust-play/3359f7575a71a077409ba0d6b16a6098

use super::frame::Frame;
use super::{error::Error, Result, debug};
use libc::{c_char, mkfifo};
use std::os::unix::ffi::OsStrExt;
use std::{
  fs::{File, OpenOptions},
  io::{self, Read, Write},
  path::{Path, PathBuf},
};

pub struct Listener {
  path: PathBuf,
}

impl Listener {
  pub fn new(path: PathBuf) -> Result<Self> {
		debug!("Instantiating listener {}", path.display());
    let os_str = path.clone().into_os_string();
    let slice = os_str.as_bytes();
    let mut bytes = Vec::with_capacity(slice.len() + 1);
    bytes.extend_from_slice(slice);
    bytes.push(0); // zero terminated string
    let _ = std::fs::remove_file(&path);
		debug!("  Removed old files");
    if unsafe { mkfifo((&bytes[0]) as *const u8 as *const c_char, 0o644) } != 0
    {
			debug!("  libc::mkfifo failed");
			debug!("--");
      Err(Error::IO(io::Error::last_os_error()))
    } else {
			debug!("  Created listener");
			debug!("--");
      Ok(Listener { path })
    }
  }
  /// Blocks until anyone connects to this fifo.
  pub fn open(&self) -> Result<Connection> {
		debug!("Listener {} waiting for connection", &self.path.display());
    let mut pipe = OpenOptions::new().read(true).open(&self.path)?;

    let mut pid_bytes = [0u8; 4];
		debug!("  Reading PID");
    pipe.read_exact(&mut pid_bytes)?;
    let pid = u32::from_ne_bytes(pid_bytes);
		debug!("  Connecting to PID {}", pid);

    drop(pipe);
		debug!("  Dropped pipe handle");

    let read = OpenOptions::new()
      .read(true)
      .open(format!("/tmp/rust-fifo-read.{}", pid))?;

    let write = OpenOptions::new()
      .write(true)
      .open(format!("/tmp/rust-fifo-write.{}", pid))?;

		debug!("  Created rust-fifo-read.{} and rust-fifo-write.{} files", pid, pid);
		debug!("  Connection ready");
		debug!("--");

    Ok(Connection { read, write })
  }
}

impl Drop for Listener {
  fn drop(&mut self) {
    let _ = std::fs::remove_file(&self.path);
  }
}

pub struct Connection {
  read: File,
  write: File,
}

pub trait Pipe<Message> {
	fn send_message(&mut self, msg: &Message) -> Result<()>;
	fn recv_message(&mut self) -> Result<Message>;
}

impl Connection {
  pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
		debug!("Opening connection from client");
    let pid = std::process::id();

    let read_fifo_path = format!("/tmp/rust-fifo-write.{}", pid);
    let read_fifo = Listener::new(read_fifo_path.into())?;

    let write_fifo_path = format!("/tmp/rust-fifo-read.{}", pid);
    let write_fifo = Listener::new(write_fifo_path.into())?;

		debug!("  Created rust-fifo-read.{} and rust-fifo-write.{} files", pid, pid);

    let mut pipe = OpenOptions::new().write(true).open(path.as_ref())?;

		debug!("  Connected to pipe");

    let pid_bytes: [u8; 4] = u32::to_ne_bytes(pid);
    pipe.write_all(&pid_bytes)?;
    pipe.flush()?;

		debug!("  Sent PID to pipe");

    let write = OpenOptions::new().write(true).open(&write_fifo.path)?;

    let read = OpenOptions::new().read(true).open(&read_fifo.path)?;

		debug!("  Connected to read and write channels");
		debug!("--");

    Ok(Self { read, write })
	}
}

impl Pipe<Frame> for Connection {
  fn send_message(&mut self, msg: &Frame) -> Result<()> {
		debug!("Called send_message");
    let msg = bincode::serialize(msg).expect("Serialization failed");
    self.write.write_all(&usize::to_ne_bytes(msg.len()))?;
		debug!("  Serialized and wrote length header");
    self.write.write_all(&msg[..])?;
    self.write.flush()?;
		debug!("  Wrote and flushed message");
		debug!("--");
    Ok(())
  }

  fn recv_message(&mut self) -> Result<Frame> {
		debug!("Called recv_message");

    let mut len_bytes = [0u8; std::mem::size_of::<usize>()];
    self.read.read_exact(&mut len_bytes)?;
    let len = usize::from_ne_bytes(len_bytes);

		debug!("  Got message header");

    let mut buf = vec![0; len];
    self.read.read_exact(&mut buf[..])?;

		debug!("  Got message contents; Deserializing");
		debug!("--");

    Ok(bincode::deserialize(&buf[..]).expect("Deserialization failed"))
  }
}

pub struct DummyConnection;

impl<M> Pipe<M> for DummyConnection where M: Default {
	fn send_message(&mut self, _: &M) -> Result<()> {
		Ok(())
	}

	fn recv_message(&mut self) -> Result<M> {
		Ok(M::default())
	}
}
