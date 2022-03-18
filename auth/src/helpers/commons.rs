use crate::errors::app::ApiError;

use super::mail::ValidEmail;
pub type Str = &'static str;


pub type ApiResult<T> = std::result::Result<T, ApiError>;

pub type DbResult<T> = Result<T, ApiError>;

pub fn forgot_password_key(id: uuid::Uuid) -> String {
    format!("forgot__{}", id)
}