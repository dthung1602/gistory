use std::fmt::{Debug, Display, Formatter};
use std::io;

use derive_more::From;

use crate::git::error::Error as GitError;

#[derive(From, Debug)]
pub enum Error {
    #[from]
    Git(GitError),
    #[from]
    Io(io::Error),
    InvalidArg(String),
    InvalidData(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
