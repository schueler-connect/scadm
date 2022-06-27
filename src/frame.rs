use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Frame {
  Stop(),
	Stopped()
}
