use std::fmt::{Debug, Display, Formatter};
use std::str::Utf8Error;
use std::string::FromUtf8Error;

use derive_more::From;
use hex::FromHexError;

#[derive(From, Debug)]
pub enum Error {
    // External error
    Utf8,
    #[from]
    Io(std::io::Error),
    #[from]
    Hex(FromHexError),
    // Internal error
    #[from]
    InvalidObjectFormat(String),
    InvalidRepoConfig(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        Self::Utf8
    }
}

impl From<Utf8Error> for Error {
    fn from(_: Utf8Error) -> Self {
        Self::Utf8
    }
}
