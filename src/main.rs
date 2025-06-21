mod error;
mod git;

use log::info;
use tokio::fs;

use crate::git::objects::Blob;

const FILE: &str = "foo/.git/objects/8c/0bf1b1f3ef6e2c4486f309728936757be620bd";
const FILE2: &str = "tmpfile";

const TREE: &str = "foo/.git/objects/22/3b8d2067d7f7f85918df7330db12dc0528da2a";
const TREE2: &str = "tmptree";

#[tokio::main]
async fn main() -> error::Result<()> {
    env_logger::init();
    info!("--> START <--");

    //    let x = git::objects::Blob::from_file(FILE).await;
    //    let blob = match x {
    //        Ok(blob) => {
    //            println!("Got content: {}", blob.content_utf8()?);
    //            blob
    //        }
    //        Err(e) => return Err(e.into()),
    //    };
    //
    //    blob.to_file(FILE2).await?;
    //
    //    let new_blob = git::objects::Blob::from_file(FILE2).await?;
    //    assert_eq!(blob.content, new_blob.content);

    let node = git::objects::Tree::from_file(TREE).await?;

    info!("--> DONE <--");
    Ok(())
}
