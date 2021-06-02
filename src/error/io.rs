use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Error(std::io::Error);

impl Clone for Error {
    fn clone(&self) -> Self {
        Self(std::io::Error::new(self.0.kind(), self.to_owned()))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "IoError: {}", self.0.to_string())
    }
}

impl std::error::Error for Error {}

impl PartialEq for Error {
    fn eq(&self, rhs: &Self) -> bool {
        // Naive implementation
        self.0.kind() == rhs.0.kind()
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self(err)
    }
}