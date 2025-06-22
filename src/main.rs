mod error;
mod git;

use log::info;

const FILE: &str = "foo/.git/objects/8c/0bf1b1f3ef6e2c4486f309728936757be620bd";
const FILE2: &str = "tmpfile";

const TREE: &str = "foo/.git/objects/22/3b8d2067d7f7f85918df7330db12dc0528da2a";
const TREE2: &str = "tmptree";

const COMMIT: &str = "foo/.git/objects/72/1b138c039c9dfa3d4d81d29c55bf6a81452020";
const COMMIT2: &str = "tmpcommit";

#[tokio::main]
async fn main() -> error::Result<()> {
    env_logger::init();
    info!("--> START <--");

    let blob = git::objects::Blob::from_file(FILE).await?;
    blob.to_file(FILE2).await?;

    let new_blob = git::objects::Blob::from_file(FILE2).await?;
    assert_eq!(blob.content, new_blob.content);

    let tree = git::objects::Tree::from_file(TREE).await?;
    tree.to_file(TREE2).await?;

    let new_tree = git::objects::Tree::from_file(TREE2).await?;
    assert_eq!(tree, new_tree);

    let commit = git::objects::Commit::from_file(COMMIT).await?;
    info!("COMMIT: {:?}", commit);

    info!("--> DONE <--");
    Ok(())
}
