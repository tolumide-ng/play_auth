use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

use crate::errors::app::ApiError;

pub type ApiResult<T> = std::result::Result<T, ApiError>;

pub type DbResult<T> = Result<T, ApiError>;

/// RedisPrefix used for prefixing redis
#[derive(Debug, PartialEq, derive_more::Display)]
pub enum RedisPrefix {
    #[display(fmt = "signup")]
    Signup,
    #[display(fmt = "login")]
    Login,
    #[display(fmt = "forgot")]
    Forgot,
    /// means login without timestamp, this is used to delete all login tokens
    #[display(fmt = "login")]
    LoginWithoutTs,
}

pub struct RedisKey {
    prefix: RedisPrefix,
    user_id: Uuid,
}

impl RedisKey {
    pub fn new(prefix: RedisPrefix, user_id: Uuid) -> Self {
        Self {
            prefix, user_id,
        }
    }

    pub fn make_key(&self) -> String {
        let mut key =  format!("{}__{}", &self.prefix, &self.user_id);

        if self.prefix == RedisPrefix::Login {
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            key = format!("{}__{}", key, time);
        }

        key
    }
}

pub const MINUTES_20: u64 = 60 * 20;
pub const MINUTES_60: u64 = 60 * 60;
pub const MINUTES_120: u64 = 60 * 60 * 2;

// pub fn get_user_from_redis_key(key: String) {}