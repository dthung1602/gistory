mod error;
mod git;

use git::prelude::*;
use log::info;

const FILE: &str = "8c0bf1b1f3ef6e2c4486f309728936757be620bd";
const TREE: &str = "223b8d2067d7f7f85918df7330db12dc0528da2a";
const COMMIT: &str = "721b138c039c9dfa3d4d81d29c55bf6a81452020";

#[tokio::main]
async fn main() -> error::Result<()> {
    env_logger::init();
    info!("--> START <--");

    let cwd = std::env::current_dir().unwrap();
    let foo = cwd.join("foo");
    let bar = cwd.join("bar");

    let foo_context = GitContext { directory: foo };
    let bar_context = GitContext { directory: bar };

    let blob = Blob::from_hex(FILE, &foo_context).await?;
    blob.write_to_file(&bar_context).await?;

    let new_blob = Blob::from_hex(FILE, &bar_context).await?;
    assert_eq!(blob, new_blob);

    let tree = Tree::from_hex(TREE, &foo_context).await?;
    tree.write_to_file(&bar_context).await?;

    let new_tree = Tree::from_hex(TREE, &bar_context).await?;
    assert_eq!(tree, new_tree);

    let commit = Commit::from_hex(COMMIT, &foo_context).await?;
    commit.write_to_file(&bar_context).await?;

    let new_commit = Commit::from_hex(COMMIT, &bar_context).await?;
    assert_eq!(commit, new_commit);

    info!("--> DONE <--");
    Ok(())
}
