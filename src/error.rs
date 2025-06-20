use std::fmt::{Debug, Display, Formatter};

use derive_more::From;

use crate::git::Error as GitError;

#[derive(From, Debug)]
pub enum Error {
    #[from]
    Git(GitError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
