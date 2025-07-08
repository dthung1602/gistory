use std::path::Path;

use chrono::NaiveDate;
use log::info;

use crate::error::{Error, Result};
use crate::git::repo::Repo;
use crate::utils::DateRangeIter;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommitCount {
    Zero = 0,
    Few = 1,
    Some = 2,
    Many = 3,
    ALot = 4,
}

impl CommitCount {
    pub fn value(self) -> usize {
        match self {
            CommitCount::Zero => 0,
            CommitCount::Few => 2,
            CommitCount::Some => 4,
            CommitCount::Many => 6,
            CommitCount::ALot => 8,
        }
    }
}

pub struct CommitGrid {
    start_date: NaiveDate,
    data: Vec<CommitCount>, // store data by date
}

impl CommitGrid {
    pub async fn populate_repo(&self, repo: &mut Repo) -> Result<()> {
        info!("Populating repo at {:?}", repo.path);
        let date_range = DateRangeIter::new(self.start_date, self.data.len());

        for (i, date) in date_range.enumerate() {
            let commit_count = self.data[i];
            info!("Creating {commit_count:?} commits for date {date:?}");
            for _ in 0..commit_count.value() {
                repo.add_commit(date).await?
            }
        }

        Ok(())
    }

    pub fn new(start_date: NaiveDate) -> Self {
        Self {
            start_date,
            data: vec![],
        }
    }

    pub fn full(&mut self, commit_count: CommitCount, end_date: NaiveDate) -> Result<()> {
        if self.start_date.gt(&end_date) {
            return Err(Error::InvalidArg(
                "end_date must be >= start_date".to_string(),
            ));
        }
        let days = (end_date - self.start_date).num_days() as usize;
        self.data = vec![commit_count; days + 1];
        Ok(())
    }

    pub fn random(&mut self, end_date: NaiveDate) -> Result<()> {
        todo!()
    }

    pub async fn read_text_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        todo!()
    }

    pub async fn read_image_file(&mut self, path: impl AsRef<Path>) -> Result<()> {
        todo!()
    }

    pub async fn show_text(&mut self, text: String) -> Result<()> {
        todo!()
    }
}
