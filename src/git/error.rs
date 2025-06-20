use std::fmt::{Debug, Display, Formatter};

use derive_more::From;

#[derive(From, Debug)]
pub enum Error {
    // External error
    #[from]
    Io(std::io::Error),
    #[from]
    FromUtf8(std::string::FromUtf8Error),
    // Internal error
    #[from]
    InvalidObject(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
