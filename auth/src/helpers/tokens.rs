use base64ct::{Base64, Encoding};
use sha2::{Sha256, Digest};
use uuid::Uuid;

use crate::errors::app::ApiError;

pub struct FingerPrint {
    data: String,
    encoded: String,
}

impl FingerPrint {
    pub fn new() -> Self {
        let data = Uuid::new_v4().to_string();
        let hash = Sha256::digest(data.clone());
        let bytes = hash.as_slice();
        let encoded = Base64::encode_string(bytes);

        Self { data, encoded }
    }

    pub fn encoded(&self) -> String {
        self.encoded
    }

    pub fn data (&self) -> String {
        self.data
    }

    pub fn with(data: String, encoded: String) -> Self {
        Self { data, encoded }
    }

    pub fn cmp(self) -> Result<Self, ApiError> {
        let decoded = Base64::decode_vec(&self.encoded)?;
        let data_bytes = self.data.clone().into_bytes();

        if decoded == data_bytes {
            return Ok(self)
        }

        return Err(ApiError::AuthenticationError("Invalid fingerprint"));
    }
}