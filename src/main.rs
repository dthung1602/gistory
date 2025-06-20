mod error;
mod git;

use log::info;

const FILE: &str = "foo/.git/objects/8c/0bf1b1f3ef6e2c4486f309728936757be620bd";

#[tokio::main]
async fn main() -> error::Result<()> {
    env_logger::init();
    info!("Hello, world!");
    let x = git::objects::Blob::from_file(FILE).await;
    match x {
        Ok(blob) => println!("Got content: {}", blob.content_utf8()?),
        Err(e) => return Err(e.into()),
    }
    Ok(())
}
