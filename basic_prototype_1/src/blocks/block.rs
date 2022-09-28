use chrono::Utc;
use sha3::{Digest, Sha3_256};

pub struct Block {
    timestamp: i64,
    pub data: String,
    pub prev_block_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &str) -> Self {
        let timestamp = Utc::now().timestamp();
        Self {
            timestamp,
            data: data.to_string(),
            prev_block_hash: prev_block_hash.to_string(),
            hash: String::from(""),
        }
    }

    pub fn set_hash(&self) -> String {
        let result = format!("{}{}{}", self.prev_block_hash, self.timestamp, self.data);
        let mut hasher = Sha3_256::new();
        hasher.update(result);
        let hash_result = hasher.finalize();
        base16ct::upper::encode_string(&hash_result)
    }
}
