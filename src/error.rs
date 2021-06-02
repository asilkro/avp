pub mod io;

use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
}