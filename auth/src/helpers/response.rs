use crate::errors::TError;

pub type TResult<T> = std::result::Result<T, TError>;