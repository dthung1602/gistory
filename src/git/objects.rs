use std::io::Write;
use std::path::{Path, PathBuf};

use atoi::FromRadix10Checked;
use flate2;
use flate2::write::{ZlibDecoder, ZlibEncoder};
use hex;
use log::debug;
use sha1::{Digest, Sha1};
use tokio::fs;
use tokio::io::{AsyncReadExt, BufReader};

use crate::git::Error::InvalidObjectFormat;
use crate::git::Result;

const READ_FILE_BUFFER_SIZE: usize = 4096;

const CONTENT_HASH_LEN: usize = 20;
type ContentHash = [u8; CONTENT_HASH_LEN];

#[derive(Debug)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl ObjectType {
    fn header(&self) -> &[u8] {
        match self {
            ObjectType::Blob => b"blob ",
            ObjectType::Tree => b"tree ",
            ObjectType::Commit => b"commit ",
        }
    }
}

pub trait Object {
    async fn read_obj_from_file(
        path: impl AsRef<Path>,
    ) -> Result<(ObjectType, Vec<u8>, ContentHash)> {
        let path_display = path.as_ref().display();
        debug!("Loading object file {path_display}");

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
        if data.is_empty() {
            return Err(InvalidObjectFormat("Empty object file".to_string()));
        }
        debug!("Done loading object file from {path_display}");

        let obj_type = match data[0] {
            b'b' => ObjectType::Blob,
            b't' => ObjectType::Tree,
            b'c' => ObjectType::Commit,
            _ => return Err(InvalidObjectFormat("Invalid header".to_string())),
        };
        let expected_header = obj_type.header();
        let header_len = expected_header.len();

        if !data.starts_with(expected_header) {
            return Err(InvalidObjectFormat(format!(
                "Invalid header. Expected '{}'",
                String::from_utf8(expected_header.into())?
            )));
        }

        // assume max file size in git is 4GiB
        let (Some(body_len), size_len) = usize::from_radix_10_checked(&data[header_len..]) else {
            return Err(InvalidObjectFormat(
                "Invalid body len in object file".to_string(),
            ));
        };

        let null_char_idx = header_len + size_len;
        match data.get(null_char_idx) {
            Some(0) => {}
            _ => {
                return Err(InvalidObjectFormat(
                    "Not found \\0 char at expected place in object file".to_string(),
                ));
            }
        }

        let remain_len = data.len() - header_len - size_len - 1;
        if remain_len != body_len {
            return Err(InvalidObjectFormat(format!(
                "Invalid data length in object file. Expected {body_len}, got {remain_len}",
            )));
        }

        debug!("Get {body_len} bytes of content from {path_display}");
        let content: Vec<u8> = data.drain(null_char_idx + 1..).collect();

        let mut hasher = Sha1::new();
        hasher.update(content.as_slice());
        let hash: ContentHash = hasher.finalize().into();
        Ok((obj_type, content, hash))
    }

    async fn write_obj_to_file(
        content: &[u8],
        obj_type: ObjectType,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        // TODO make this true async
        let path_display = path.as_ref().display();
        debug!("Writing object to file {path_display}");

        let file = std::fs::File::create(&path)?;
        let writer = std::io::BufWriter::new(file);
        let mut encoder = ZlibEncoder::new(writer, flate2::Compression::default());
        encoder.write_all(obj_type.header())?;
        encoder.write_all(format!("{}\0", content.len()).as_bytes())?;
        encoder.write_all(content)?;
        encoder.flush()?;
        let total_written = encoder.total_out();
        let compress_ratio = 100.0 - 100.0 * total_written as f64 / encoder.total_in() as f64;
        encoder.finish()?;
        debug!(
            "Done writing {} bytes to file {}. Compress ratio {:.2}%",
            total_written, path_display, compress_ratio
        );
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Blob {
    pub(crate) content: Vec<u8>,
    pub(crate) hash: ContentHash,
}

impl Object for Blob {}

impl Blob {
    pub async fn from_hash(hash: impl AsRef<ContentHash>) -> Result<Self> {
        let hash = hex::encode(hash.as_ref());
        let mut path_buf = PathBuf::from(".git/objects");
        path_buf.push(&hash[0..2]);
        path_buf.push(&hash[2..]);

        Self::from_file(&path_buf).await
    }

    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let (obj_type, content, hash) = Self::read_obj_from_file(path).await?;
        match obj_type {
            ObjectType::Blob => Ok(Self { content, hash }),
            _ => Err(InvalidObjectFormat(format!(
                "Expected blob, found {:?}",
                obj_type
            ))),
        }
    }

    pub async fn to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        Self::write_obj_to_file(&self.content, ObjectType::Blob, path).await
    }

    pub fn content_utf8(&self) -> Result<String> {
        Ok(String::from_utf8(self.content.clone())?)
    }
}

#[derive(Debug, PartialEq)]
pub enum TreeNodeMode {
    Regular,
    Executable,
    Directory,
    // Symlink,
    // Submodule,
}

impl TreeNodeMode {
    fn from_bytes(content: &[u8]) -> Result<(Self, usize)> {
        if let (Some(mode), used_bytes) = usize::from_radix_10_checked(content) {
            let tree_mode = match mode {
                100644 => Self::Regular,
                100755 => Self::Executable,
                40000 => Self::Directory,
                120000 | 160000 => {
                    return Err(InvalidObjectFormat(
                        "Not supported symlink & submodule".to_string(),
                    ));
                }
                _ => return Err(InvalidObjectFormat(format!("Invalid file mode: {mode}"))),
            };
            Ok((tree_mode, used_bytes))
        } else {
            Err(InvalidObjectFormat("Invalid file mode".to_string()))
        }
    }

    fn to_bytes(&self) -> &[u8] {
        match self {
            TreeNodeMode::Regular => b"100644",
            TreeNodeMode::Executable => b"100755",
            TreeNodeMode::Directory => b"40000",
        }
    }
}

impl Default for TreeNodeMode {
    fn default() -> Self {
        Self::Regular
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct TreeNode {
    pub(crate) mode: TreeNodeMode,
    pub(crate) name: String,
    pub(crate) hash: ContentHash,
}

#[derive(Debug, Default, PartialEq)]
pub struct Tree {
    pub(crate) nodes: Vec<TreeNode>,
    pub(crate) hash: ContentHash,
}

impl Object for Tree {}

impl Tree {
    pub async fn from_file(path: impl AsRef<Path>) -> Result<Tree> {
        let path_display = path.as_ref().display();
        let (obj_type, content, hash) = Self::read_obj_from_file(&path).await?;
        match obj_type {
            ObjectType::Tree => {}
            _ => {
                return Err(InvalidObjectFormat(format!(
                    "Expected tree, found {:?} in {}",
                    obj_type, path_display,
                )));
            }
        }

        debug!("Parsing tree nodes");
        let mut parse_idx = 0;
        let content_len = content.len();
        let mut nodes = Vec::new();
        while parse_idx < content_len {
            let (node, parsed_byte_count) = Self::parse_node(&content[parse_idx..])?;
            parse_idx += parsed_byte_count;
            nodes.push(node);
        }

        Ok(Tree { nodes, hash })
    }

    fn parse_node(content: &[u8]) -> Result<(TreeNode, usize)> {
        let (mode, mode_byte_count) = TreeNodeMode::from_bytes(content)?;
        match content.get(mode_byte_count) {
            Some(b' ') => {}
            _ => {
                return Err(InvalidObjectFormat(
                    "Not found space character at expected place in tree node".to_string(),
                ));
            }
        }

        let name_start_idx = mode_byte_count + 1;
        let name_len = content[name_start_idx..]
            .iter()
            .take_while(|x| **x != 0)
            .count();
        if name_len == 0 {
            return Err(InvalidObjectFormat("Empty name for tree node".to_string()));
        }
        let name_end_idx = name_start_idx + name_len;
        let name = String::from_utf8(content[name_start_idx..name_end_idx].to_vec())?;

        match content.get(name_end_idx) {
            Some(0) => {}
            _ => {
                return Err(InvalidObjectFormat(
                    "Not found \\0 character at expected place in tree node".to_string(),
                ));
            }
        }

        let hash_start_idx = name_end_idx + 1;
        let hash_end_idx = hash_start_idx + CONTENT_HASH_LEN;
        if hash_end_idx > content.len() {
            return Err(InvalidObjectFormat(
                "Not enough data for hash in tree node".to_string(),
            ));
        }
        let mut hash = ContentHash::default();
        hash.copy_from_slice(&content[hash_start_idx..hash_end_idx]);

        debug!("Found node {name} ({hash_end_idx} bytes)");
        Ok((TreeNode { mode, name, hash }, hash_end_idx))
    }

    pub async fn to_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let estimated_len = self.nodes.len() * (6 + 1 + 36 + 1 + 20); // assume a file name is on avg 36 bytes
        let mut content: Vec<u8> = Vec::with_capacity(estimated_len);

        for node in &self.nodes {
            // assume nodes are already sorted by name
            let mode = node.mode.to_bytes();
            content.extend_from_slice(mode);
            content.push(b' ');
            content.extend_from_slice(node.name.as_bytes());
            content.push(0);
            content.extend_from_slice(&node.hash);
        }

        Self::write_obj_to_file(&content, ObjectType::Tree, path).await
    }
}
