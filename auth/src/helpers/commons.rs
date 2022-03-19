use crate::errors::app::ApiError;

pub type Str = &'static str;

pub type ApiResult<T> = std::result::Result<T, ApiError>;

pub type DbResult<T> = Result<T, ApiError>;

pub fn make_redis_key(prefix: &'static str, id: uuid::Uuid) -> String {
    format!("{}__{}", prefix, id)
}

// pub fn get_user_from_redis_key(key: String) {}