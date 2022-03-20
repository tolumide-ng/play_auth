use sha2::{Sha256, Digest};
use uuid::Uuid;

pub struct Token {
    data: String,
    hash: String,
}

impl Token {
    pub fn new() {
        let data = Uuid::new_v4().to_string();
        let hash = Sha256::digest(data);
        println!("{:#?}", hash);
        // Self {data, hasher}
    }

    pub fn with(data: String, hash: String) -> Self {
        Self {data, hash}
    }

    pub fn cmp(&self) {}
}