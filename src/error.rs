use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

use derive_more::From;

use crate::git::error::Error as GitError;

#[derive(From, Debug)]
pub enum Error {
    #[from]
    Git(GitError),
    #[from]
    Io(io::Error),
    Utf8(String),
    InvalidArg(String),
    InvalidData(String),
    Command(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        Self::Utf8(value.to_string())
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8(value.to_string())
    }
}
