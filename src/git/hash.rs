use sha1::{Digest, Sha1};

pub const CONTENT_HASH_LEN: usize = 20;
pub const HEX_CONTENT_HASH_LEN: usize = CONTENT_HASH_LEN * 2;

#[derive(Debug, Default, PartialEq)]
pub struct ContentHash {
    pub(crate) value: [u8; CONTENT_HASH_LEN],
}

impl ContentHash {
    pub fn from_slice(slice: &[u8]) -> Self {
        let mut content_hash = Self::default();
        content_hash.value.copy_from_slice(slice);
        content_hash
    }
}

impl From<[u8; CONTENT_HASH_LEN]> for ContentHash {
    fn from(value: [u8; CONTENT_HASH_LEN]) -> Self {
        Self { value }
    }
}

impl TryFrom<&str> for ContentHash {
    type Error = crate::git::error::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let hex_str = hex::decode(value)?;
        let value: [u8; CONTENT_HASH_LEN] = hex_str
            .try_into()
            .map_err(|_| hex::FromHexError::InvalidStringLength)?;
        Ok(Self { value })
    }
}

impl ToString for ContentHash {
    fn to_string(&self) -> String {
        hex::encode(&self.value)
    }
}

pub fn calculate_content_hash(content: &[u8]) -> ContentHash {
    let mut hasher = Sha1::new();
    hasher.update(&content);
    ContentHash {
        value: hasher.finalize().into(),
    }
}
