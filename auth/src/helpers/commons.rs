use crate::errors::app::ApiError;
pub type Str = &'static str;


pub type ApiResult<T> = std::result::Result<T, ApiError>;

pub type DbResult<T> = Result<T, ApiError>;