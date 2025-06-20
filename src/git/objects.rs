use std::io::Write;
use std::path::Path;

use atoi::FromRadix10Checked;
use flate2;
use flate2::write::{ZlibDecoder, ZlibEncoder};
use log::debug;
use tokio::fs;
use tokio::io::{AsyncReadExt, BufReader};

use crate::git::Error::InvalidObject;
use crate::git::Result;

const READ_FILE_BUFFER_SIZE: usize = 4096;

const BLOB_HEADER: &[u8] = b"blob ";
const TREE_HEADER: &[u8] = b"tree ";
const COMMIT_HEADER: &[u8] = b"commit ";

#[derive(Debug, Default)]
pub struct Blob {
    pub(crate) content: Vec<u8>,
}

impl Blob {
    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        debug!("Loading blob from {}", path.as_ref().display());

        let data: Vec<u8> = Vec::with_capacity(READ_FILE_BUFFER_SIZE);
        let mut decoder = ZlibDecoder::new(data);
        let mut buff: [u8; READ_FILE_BUFFER_SIZE] = [0; READ_FILE_BUFFER_SIZE];
        let file = fs::File::open(&path).await?;
        let mut reader = BufReader::new(file);

        loop {
            let size = reader.read(&mut buff).await?;
            if size == 0 {
                break;
            }
            decoder.write_all(&buff[..size])?;
        }
        let mut data = decoder.finish()?;
        debug!("Done loading blob from {}", path.as_ref().display());

        match &data[0..5] {
            BLOB_HEADER => {}
            TREE_HEADER => {
                return Err(InvalidObject(
                    "Expecting blob, found tree object".to_string(),
                ));
            }
            COMMIT_HEADER => {
                return Err(InvalidObject(
                    "Expecting commit, found tree object".to_string(),
                ));
            }
            _ => {
                return Err(InvalidObject(
                    "Invalid type header in object file".to_string(),
                ));
            }
        }

        // assume max file size in git is 4GiB
        let (Some(body_len), bytes_read) = usize::from_radix_10_checked(&data[5..]) else {
            return Err(InvalidObject("Invalid body len in object file".to_string()));
        };

        let null_char_idx = 5 + bytes_read;
        if data[null_char_idx] != 0 {
            return Err(InvalidObject(
                "Invalid \\0 character in object file".to_string(),
            ));
        }

        let remain_len = data.len() - 5 - bytes_read - 1;
        if remain_len != body_len {
            return Err(InvalidObject(format!(
                "Invalid data length in object file. Expected {body_len}, got {remain_len}",
            )));
        }

        debug!(
            "Uncompressed content len from {}: {}",
            path.as_ref().display(),
            body_len
        );
        Ok(Self {
            content: data.drain(null_char_idx + 1..).collect(),
        })
    }

    pub async fn to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        // TODO make this true async
        debug!("Writing blob to {}", path.as_ref().display());
        let file = std::fs::File::create(&path)?;
        let writer = std::io::BufWriter::new(file);
        let mut encoder = ZlibEncoder::new(writer, flate2::Compression::default());
        encoder.write_all(BLOB_HEADER)?;
        encoder.write_all(format!("{}\0", self.content.len()).as_bytes())?;
        encoder.write_all(&self.content)?;
        encoder.flush()?;
        let total_written = encoder.total_out();
        let compress_ratio = 100.0 - 100.0 * total_written as f64 / encoder.total_in() as f64;
        encoder.finish()?;
        debug!(
            "Done writing {} bytes to blob {}. Compress ratio {:.2}%",
            total_written,
            path.as_ref().display(),
            compress_ratio
        );
        Ok(())
    }

    pub fn content_utf8(&self) -> Result<String> {
        Ok(String::from_utf8(self.content.clone())?)
    }
}
