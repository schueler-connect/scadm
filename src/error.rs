#[derive(Debug)]
pub enum Error {
	WithMessage(String),
	IO(tokio::io::Error),
	Parsing,
	Unknown
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<&str> for Error {
	fn from(e: &str) -> Error {
		Error::WithMessage(e.to_owned())
	}
}

impl From<tokio::io::Error> for Error {
	fn from(e: tokio::io::Error) -> Error {
		Error::IO(e)
	}
}
