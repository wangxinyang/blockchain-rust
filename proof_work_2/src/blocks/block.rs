use chrono::Utc;

pub struct Block {
    pub timestamp: i64,
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

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash
    }
}
