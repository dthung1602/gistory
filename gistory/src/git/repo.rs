use std::path::PathBuf;

use chrono::{FixedOffset, NaiveDate, TimeZone};
use log::debug;
use tokio::fs;

use super::error::{Error, Result};
use super::hash::ContentHash;
use super::objects::{Commit, Object, Tree};
use crate::git::objects::User;

const CONFIG_FILE_CONTENT: &str = "[core]\n\trepositoryformatversion = 0\n\tfilemode = true\n\tbare = false\n\tlogallrefupdates = true\n";

#[derive(Debug)]
pub struct Repo {
    pub path: PathBuf,
    pub default_branch: String,
    pub time_zone: FixedOffset,
    pub user_name: String,
    pub email: String,
    pub commits: Vec<Commit>,
}

impl Repo {
    pub fn new(
        path: PathBuf,
        default_branch: String,
        time_zone: FixedOffset,
        user_name: String,
        email: String,
    ) -> Self {
        Self {
            path,
            default_branch,
            time_zone,
            user_name,
            email,
            commits: vec![],
        }
    }

    pub fn git_directory(&self) -> PathBuf {
        self.path.join(".git")
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

    pub async fn init(&self) -> Result<()> {
        let git_dir = self.git_directory();

        if fs::try_exists(&git_dir).await? {
            debug!("Clearing git dir at {}", git_dir.display());
            fs::remove_dir_all(&git_dir).await?
        }

        debug!("Creating git dir at {}", git_dir.display());
        fs::create_dir_all(&git_dir).await?;

        // Create HEAD
        let head_file = git_dir.join("HEAD");
        fs::write(
            head_file,
            format!("ref: refs/heads/{}\n", self.default_branch),
        )
        .await?;

        // Create config
        let config_file = git_dir.join("config");
        fs::write(config_file, CONFIG_FILE_CONTENT).await?;

        // object folder
        let object_dir = git_dir.join("objects");
        let obj_info_dir = object_dir.join("info");
        let obj_pack_dir = object_dir.join("pack");
        fs::create_dir_all(obj_info_dir).await?;
        fs::create_dir_all(obj_pack_dir).await?;

        // refs folder
        let ref_dir = git_dir.join("refs");
        let ref_head_dir = ref_dir.join("heads");
        let ref_tag_dir = ref_dir.join("tags");
        fs::create_dir_all(ref_head_dir).await?;
        fs::create_dir_all(ref_tag_dir).await?;

        Ok(())
    }

    pub async fn add_commit(&mut self, date: NaiveDate) -> Result<()> {
        let naive_date_time = date.and_hms_opt(11, 11, 11).unwrap();
        let Some(date_time) = self
            .time_zone
            .from_local_datetime(&naive_date_time)
            .earliest()
        else {
            return Err(Error::InvalidRepoConfig(
                "Cannot convert datetime".to_string(),
            ));
        };

        let author = User {
            name: self.user_name.clone(),
            email: self.email.clone(),
            date_time,
        };
        let committer = author.clone();

        let message = format!("Commit #{}", self.commits.len());

        let parents = match self.commits.last() {
            Some(last_commit) => vec![last_commit.get_hash().clone()],
            None => vec![],
        };

        let tree = Tree::default();
        tree.write_to_file(self).await?;
        let tree_hash = tree.get_hash().clone();

        let commit = Commit::new(tree_hash, parents, author, committer, message);
        commit.write_to_file(self).await?;

        let ref_master_path = self
            .git_directory()
            .join("refs/heads/")
            .join(&self.default_branch);
        let hash_str = commit.get_hash().to_string();
        fs::write(ref_master_path, format!("{hash_str}\n")).await?;

        self.commits.push(commit);

        Ok(())
    }
}
