use std::path::PathBuf;

use crate::git::error::Result;
use crate::git::hash::ContentHash;

pub struct GitContext {
    pub directory: PathBuf,
}

impl GitContext {
    pub fn git_directory(&self) -> PathBuf {
        self.directory.join(".git")
    }

    pub fn obj_path_from_hash(&self, hash: &ContentHash) -> PathBuf {
        let hex = hash.to_string();
        let mut path_buf = self.git_directory();
        path_buf.push("objects");
        path_buf.push(&hex[0..2]);
        path_buf.push(&hex[2..]);
        path_buf
    }

    pub fn obj_path_from_hex(&self, hex: &str) -> Result<PathBuf> {
        let hash: ContentHash = hex.try_into()?;
        Ok(self.obj_path_from_hash(&hash))
    }
}

impl Default for GitContext {
    fn default() -> Self {
        let cwd = std::env::current_dir().expect("Cannot get CWD");
        Self { directory: cwd }
    }
}
