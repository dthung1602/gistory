mod error;
mod git;

use crate::git::objects::Blob;
use log::info;
use tokio::fs;

const FILE: &str = "foo/.git/objects/8c/0bf1b1f3ef6e2c4486f309728936757be620bd";
const FILE2: &str = "tmpfile";

#[tokio::main]
async fn main() -> error::Result<()> {
    env_logger::init();
    info!("Hello, world!");

    let x = git::objects::Blob::from_file(FILE).await;
    let blob = match x {
        Ok(blob) => {
            println!("Got content: {}", blob.content_utf8()?);
            blob
        }
        Err(e) => return Err(e.into()),
    };

    blob.to_file(FILE2).await?;

    let new_blob = git::objects::Blob::from_file(FILE2).await?;
    assert_eq!(blob.content, new_blob.content);
    Ok(())
}
