use std::fmt::Debug;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use atoi::FromRadix10Checked;
use chrono::{DateTime, FixedOffset, TimeZone};
use flate2;
use flate2::write::{ZlibDecoder, ZlibEncoder};
use log::debug;
use tokio::io::{AsyncReadExt, BufReader};

use crate::git::error::Error::InvalidObjectFormat;
use crate::git::error::Result;
use crate::git::hash::{
    CONTENT_HASH_LEN, ContentHash, HEX_CONTENT_HASH_LEN, calculate_content_hash,
};
use crate::git::repo::Repo;

// region Common

pub trait Object: Sized + PartialEq {
    fn get_hash(&self) -> &ContentHash;

    async fn from_hex(hex: &str, repo: &Repo) -> Result<Self>;

    async fn write_to_file(&self, repo: &Repo) -> Result<usize>;
}

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

const READ_FILE_BUFFER_SIZE: usize = 4096;

async fn read_obj_from_file(path: impl AsRef<Path>) -> Result<(ObjectType, Vec<u8>)> {
    let path_display = path.as_ref().display();
    debug!("Loading object file {path_display}");

    let data: Vec<u8> = Vec::with_capacity(READ_FILE_BUFFER_SIZE);
    let mut decoder = ZlibDecoder::new(data);
    let mut buff: [u8; READ_FILE_BUFFER_SIZE] = [0; READ_FILE_BUFFER_SIZE];
    let file = tokio::fs::File::open(&path).await?;
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

    Ok((obj_type, content))
}

fn format_content(content: &[u8], obj_type: ObjectType) -> (Vec<u8>, ContentHash) {
    let mut formatted_content: Vec<u8> = Vec::with_capacity(content.len() + 32);
    formatted_content.extend_from_slice(obj_type.header());
    formatted_content.extend_from_slice(format!("{}\0", content.len()).as_bytes());
    formatted_content.extend_from_slice(content);
    let hash = calculate_content_hash(&formatted_content);
    (formatted_content, hash)
}

async fn write_obj_to_file(formatted_content: &[u8], path: impl AsRef<Path>) -> Result<usize> {
    // TODO make this true async
    let path_display = path.as_ref().display();
    debug!("Writing object to file {path_display}");

    // ensure parent dirs exists
    let parent = path.as_ref().parent().unwrap();
    tokio::fs::create_dir_all(parent).await?;

    // write zlib compressed data
    let file = std::fs::File::create(&path)?;
    let writer = std::io::BufWriter::new(file);
    let mut encoder = ZlibEncoder::new(writer, flate2::Compression::default());
    encoder.write_all(formatted_content)?;
    encoder.flush()?;
    let total_written = encoder.total_out();
    let compress_ratio = 100.0 - 100.0 * total_written as f64 / encoder.total_in() as f64;
    encoder.finish()?;
    debug!(
        "Done writing {total_written} bytes to file {path_display}. Compress ratio {compress_ratio:.2}%"
    );
    Ok(total_written as usize)
}

// endregion

// region Blob

#[derive(Debug, PartialEq)]
pub struct Blob {
    content: Vec<u8>,
    hash: ContentHash,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        let (_, hash) = format_content(&content, ObjectType::Blob);
        Self { content, hash }
    }

    pub fn get_content(&self) -> &[u8] {
        &self.content
    }
}

impl Object for Blob {
    fn get_hash(&self) -> &ContentHash {
        &self.hash
    }

    async fn from_hex(hex: &str, repo: &Repo) -> Result<Self> {
        let path = repo.obj_path_from_hex(hex)?;
        let path_display = path.display();

        let (obj_type, content) = read_obj_from_file(&path).await?;

        match obj_type {
            ObjectType::Blob => {
                let hash = hex.try_into()?;
                Ok(Self { content, hash })
            }
            _ => Err(InvalidObjectFormat(format!(
                "Expected blob, found {obj_type:?} in {path_display}"
            ))),
        }
    }

    async fn write_to_file(&self, repo: &Repo) -> Result<usize> {
        let path = repo.obj_path_from_hash(&self.hash);
        let (formatted_content, _) = format_content(&self.content, ObjectType::Blob);
        write_obj_to_file(&formatted_content, path).await
    }
}

// endregion

// region Tree

#[derive(Debug, PartialEq)]
pub enum TreeNodeMode {
    Regular,
    Executable,
    Directory,
    // Symlink,
    // Submodule,
}

impl TreeNodeMode {
    fn parse(content: &[u8]) -> Result<(Self, usize)> {
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

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub mode: TreeNodeMode,
    pub name: String,
    pub hash: ContentHash,
}

#[derive(Debug, PartialEq)]
pub struct Tree {
    nodes: Vec<TreeNode>,
    hash: ContentHash,
}

impl Object for Tree {
    fn get_hash(&self) -> &ContentHash {
        &self.hash
    }

    async fn from_hex(hex: &str, repo: &Repo) -> Result<Self> {
        let path = repo.obj_path_from_hex(hex)?;
        let path_display = path.display();

        let (obj_type, content) = read_obj_from_file(&path).await?;
        match obj_type {
            ObjectType::Tree => {
                let nodes = Tree::parse_nodes(&content)?;
                let hash = hex.try_into()?;
                Ok(Self { nodes, hash })
            }
            _ => Err(InvalidObjectFormat(format!(
                "Expected tree, found {obj_type:?} in {path_display}",
            ))),
        }
    }

    async fn write_to_file(&self, repo: &Repo) -> Result<usize> {
        let path = repo.obj_path_from_hash(&self.hash);
        let bytes = Self::nodes_to_bytes(&self.nodes);
        let (formated_content, _) = format_content(&bytes, ObjectType::Tree);
        write_obj_to_file(&formated_content, path).await
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self {
            nodes: vec![],
            hash: "4b825dc642cb6eb9a060e54bf8d69288fbee4904"
                .try_into()
                .unwrap(),
        }
    }
}

impl Tree {
    pub fn new(nodes: Vec<TreeNode>) -> Self {
        let bytes = Self::nodes_to_bytes(&nodes);
        let (_, hash) = format_content(&bytes, ObjectType::Tree);
        Self { nodes, hash }
    }

    pub fn get_nodes(&self) -> &[TreeNode] {
        &self.nodes
    }

    fn parse_nodes(content: &[u8]) -> Result<Vec<TreeNode>> {
        let mut parse_idx = 0;
        let content_len = content.len();
        let mut nodes = Vec::new();
        while parse_idx < content_len {
            let (node, parsed_byte_count) = Self::parse_node(&content[parse_idx..])?;
            parse_idx += parsed_byte_count;
            nodes.push(node);
        }

        Ok(nodes)
    }

    fn nodes_to_bytes(nodes: &[TreeNode]) -> Vec<u8> {
        let estimated_len = nodes.len() * (6 + 1 + 36 + 1 + 20); // assume a file name is on avg 36 bytes
        let mut bytes: Vec<u8> = Vec::with_capacity(estimated_len);

        for node in nodes {
            // assume nodes are already sorted by name
            let mode = node.mode.to_bytes();
            bytes.extend_from_slice(mode);
            bytes.push(b' ');
            bytes.extend_from_slice(node.name.as_bytes());
            bytes.push(0);
            bytes.extend_from_slice(&node.hash.value);
        }

        bytes
    }

    fn parse_node(content: &[u8]) -> Result<(TreeNode, usize)> {
        let (mode, mode_byte_count) = TreeNodeMode::parse(content)?;
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
        let hash = ContentHash::from_slice(&content[hash_start_idx..hash_end_idx]);

        debug!("Found node {name} ({hash_end_idx} bytes)");
        Ok((TreeNode { mode, name, hash }, hash_end_idx))
    }
}

// endregion

// region Commit

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub date_time: DateTime<FixedOffset>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Commit {
    hash: ContentHash,
    tree: ContentHash,
    parents: Vec<ContentHash>,
    author: User,
    committer: User,
    message: String,
}

impl Object for Commit {
    fn get_hash(&self) -> &ContentHash {
        &self.hash
    }

    async fn from_hex(hex: &str, repo: &Repo) -> Result<Self> {
        let path = repo.obj_path_from_hex(hex)?;
        let path_display = path.display();

        let (obj_type, content) = read_obj_from_file(&path).await?;
        match obj_type {
            ObjectType::Commit => {}
            _ => {
                return Err(InvalidObjectFormat(format!(
                    "Expected commit, found {obj_type:?} in {path_display}",
                )));
            }
        };

        let (tree_hash, mut total_read_bytes) = Self::parse_hash(b"tree ", &content)?;

        let mut parents_hash = Vec::new();
        while let Ok((parent_hash, parent_read_bytes)) =
            Self::parse_hash(b"parent ", &content[total_read_bytes..])
        {
            parents_hash.push(parent_hash);
            total_read_bytes += parent_read_bytes;
        }

        let (author, author_read_bytes) =
            Self::parse_user(b"author ", &content[total_read_bytes..])?;
        total_read_bytes += author_read_bytes;

        let (committer, committer_read_bytes) =
            Self::parse_user(b"committer ", &content[total_read_bytes..])?;
        total_read_bytes += committer_read_bytes;

        // we dont support gpgsig

        match content.get(total_read_bytes) {
            Some(b'\n') => {}
            _ => {
                return Err(InvalidObjectFormat(
                    "Not found \\n character at expected place in commit object".to_string(),
                ));
            }
        }

        // the message is the rest of the file, except for the last char, which is \n
        let message = String::from_utf8(content[total_read_bytes + 1..content.len() - 1].to_vec())?;

        Ok(Commit {
            hash: hex.try_into()?,
            tree: tree_hash,
            parents: parents_hash,
            author,
            committer,
            message,
        })
    }

    async fn write_to_file(&self, repo: &Repo) -> Result<usize> {
        let path = repo.obj_path_from_hash(&self.hash);
        let bytes = self.to_bytes();
        let (formatted_content, _) = format_content(&bytes, ObjectType::Commit);
        write_obj_to_file(&formatted_content, path).await
    }
}

impl Commit {
    pub fn new(
        tree: ContentHash,
        parents: Vec<ContentHash>,
        author: User,
        committer: User,
        message: String,
    ) -> Self {
        let mut commit = Self {
            hash: ContentHash::default(),
            tree,
            parents,
            author,
            committer,
            message,
        };
        let bytes = commit.to_bytes();
        let (_, hash) = format_content(&bytes, ObjectType::Commit);
        commit.hash = hash;
        commit
    }

    fn parse_hash(header: &[u8], content: &[u8]) -> Result<(ContentHash, usize)> {
        if !content.starts_with(header) {
            return Err(InvalidObjectFormat(format!(
                "Cannot find header {} in commit object",
                String::from_utf8(header.to_vec())?
            )));
        }

        let header_len = header.len();
        let end_idx = header_len + HEX_CONTENT_HASH_LEN;
        if end_idx > content.len() {
            return Err(InvalidObjectFormat(format!(
                "Cannot parse header {} of commit object",
                String::from_utf8(header.to_vec())?
            )));
        }

        let hash_str = str::from_utf8(&content[header_len..end_idx])?;
        let hash = ContentHash::try_from(hash_str)?;

        Ok((hash, end_idx + 1))
    }

    fn parse_user(header: &[u8], content: &[u8]) -> Result<(User, usize)> {
        if !content.starts_with(header) {
            return Err(InvalidObjectFormat(format!(
                "Cannot find header {} in commit object",
                String::from_utf8(header.to_vec())?
            )));
        }

        let Some((email_start_idx, _)) = content.iter().enumerate().find(|(_, x)| **x == b'<')
        else {
            return Err(InvalidObjectFormat(format!(
                "Cannot find < while parsing {} in commit object",
                String::from_utf8(header.to_vec())?
            )));
        };
        let Some((email_end_idx, _)) = content.iter().enumerate().find(|(_, x)| **x == b'>') else {
            return Err(InvalidObjectFormat(format!(
                "Cannot find > while parsing {} in commit object",
                String::from_utf8(header.to_vec())?
            )));
        };
        if email_start_idx + 1 >= email_end_idx {
            return Err(InvalidObjectFormat(
                "Invalid email marker in commit object".to_string(),
            ));
        }

        let name = String::from_utf8(content[header.len()..email_start_idx].to_vec())?
            .trim()
            .to_string();
        let email = String::from_utf8(content[email_start_idx + 1..email_end_idx].to_vec())?
            .trim()
            .to_string();

        let Some((end_of_line, _)) = content.iter().enumerate().find(|(_, x)| **x == b'\n') else {
            return Err(InvalidObjectFormat(format!(
                "Cannot find \\n while parsing {} in commit object",
                String::from_utf8(header.to_vec())?
            )));
        };
        let timestamp_str =
            String::from_utf8(content[email_end_idx + 1..end_of_line + 1].to_vec())?
                .trim()
                .to_string();

        let parts: Vec<&str> = timestamp_str.splitn(2, " ").collect();
        if parts.len() != 2 {
            return Err(InvalidObjectFormat(
                "Cannot parse commit timestamp".to_string(),
            ));
        }

        let timestamp_seconds: i64 = parts[0]
            .parse()
            .map_err(|_| InvalidObjectFormat("Cannot parse commit timestamp".to_string()))?;
        let offset = FixedOffset::from_str(parts[1])
            .map_err(|_| InvalidObjectFormat("Failed to parse timezone offset".to_string()))?;

        let Some(date_time) = offset.timestamp_opt(timestamp_seconds, 0).earliest() else {
            return Err(InvalidObjectFormat(
                "Cannot parse timestamp and timezone".to_string(),
            ));
        };

        let total_read_bytes = end_of_line + 1;
        let user = User {
            name,
            email,
            date_time,
        };
        Ok((user, total_read_bytes))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut content = Vec::with_capacity(512); // estimated

        Self::write_hash(&mut content, b"tree ", &self.tree);
        for parent in &self.parents {
            Self::write_hash(&mut content, b"parent ", parent);
        }

        Self::write_user(&mut content, b"author ", &self.author);
        Self::write_user(&mut content, b"committer ", &self.committer);

        content.push(b'\n');
        content.extend(self.message.as_bytes());
        content.push(b'\n');
        content
    }

    fn write_hash(buffer: &mut Vec<u8>, header: &[u8], content_hash: &ContentHash) {
        buffer.extend(header);
        buffer.extend(content_hash.to_string().as_bytes());
        buffer.push(b'\n');
    }

    fn write_user(buffer: &mut Vec<u8>, header: &[u8], user: &User) {
        buffer.extend(header);
        buffer.extend(user.name.as_bytes());
        buffer.extend(b" <");
        buffer.extend(user.email.as_bytes());
        let date_str = user.date_time.format("> %s %z\n").to_string();
        buffer.extend(date_str.as_bytes());
    }
}

// endregion
