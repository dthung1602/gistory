mod error;
mod git;
mod utils;
mod visualizer;

use chrono::{FixedOffset, NaiveDate};
use log::info;

use crate::visualizer::CommitCount;
// const FILE: &str = "8c0bf1b1f3ef6e2c4486f309728936757be620bd";
// const TREE: &str = "223b8d2067d7f7f85918df7330db12dc0528da2a";
// const COMMIT: &str = "721b138c039c9dfa3d4d81d29c55bf6a81452020";

#[tokio::main]
async fn main() -> error::Result<()> {
    env_logger::init();
    info!("--> START <--");

    let start_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2025, 1, 7).unwrap();

    let time_zone = FixedOffset::west_opt(0).unwrap(); // github uses UTC
    let default_branch = "master";
    let user_name = "john";
    let email = "abc@example.com";

    let cwd = std::env::current_dir().unwrap();

    let mut his_repo = git::repo::Repo::new(
        cwd.join("his"),
        default_branch.to_string(),
        time_zone,
        user_name.to_string(),
        email.to_string(),
    );

    his_repo.init().await?;

    let mut grid = visualizer::CommitGrid::new(start_date);
    grid.full(CommitCount::Many, end_date)?;

    grid.populate_repo(&mut his_repo).await?;

    // let foo = cwd.join("foo");
    // let bar = cwd.join("bar");

    // let foo_repo = Repo::new(
    //     foo,
    //     default_branch.to_string(),
    //     time_zone.clone(),
    //     user_name.to_string(),
    //     email.to_string(),
    // );
    // let bar_repo = Repo::new(
    //     bar,
    //     default_branch.to_string(),
    //     time_zone,
    //     user_name.to_string(),
    //     email.to_string(),
    // );
    //
    // let blob = Blob::from_hex(FILE, &foo_repo).await?;
    // blob.write_to_file(&bar_repo).await?;
    //
    // let new_blob = Blob::from_hex(FILE, &bar_repo).await?;
    // assert_eq!(blob, new_blob);
    //
    // let tree = Tree::from_hex(TREE, &foo_repo).await?;
    // tree.write_to_file(&bar_repo).await?;
    //
    // let new_tree = Tree::from_hex(TREE, &bar_repo).await?;
    // assert_eq!(tree, new_tree);
    //
    // let commit = Commit::from_hex(COMMIT, &foo_repo).await?;
    // commit.write_to_file(&bar_repo).await?;
    //
    // let new_commit = Commit::from_hex(COMMIT, &bar_repo).await?;
    // assert_eq!(commit, new_commit);

    info!("--> DONE <--");
    Ok(())
}
