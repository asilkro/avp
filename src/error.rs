pub mod io;

use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("{0}")]
    SerializationError(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        io::Error::from(err).into()
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerializationError(err.to_string())
    }
}