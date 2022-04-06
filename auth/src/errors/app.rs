use base64ct::Error as B64Error;
use jsonwebtoken::errors::Error as JwtError;
use thiserror::Error;
use rocket::{
    http::Status,
    response::{self, Responder},
    serde::json::Json,
    Request, Response,
};
use redis::RedisError;
use serde::{Deserialize, Serialize};
use argon2::Error as PError;
// use argon2::password_hash::Error;

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorResponse {
    #[serde(rename = "error")]
    Response {
        status: u16,
        message: &'static str,
        #[serde(skip_serializing_if = "Option::is_none")]
        body: Option<String>,
    }
}


#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Internal Server Error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Token is either expired or invalid")]
    JwtError(#[from] JwtError),
    #[error("{0}")]
    BadRequest(&'static str),
    #[error("{0}")]
    ValidationError(&'static str),
    #[error("{0}")]
    Conflict(&'static str),
    #[error("{0}")]
    AuthenticationError(&'static str),
    #[error("{0}")]
    AuthorizationError(&'static str),
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Internal Server Error")]
    PasswordError(#[from] PError),
    #[error("Internal Server Error")]
    RedisError(#[from] RedisError),
    #[error("Please verify your account by clicking the email sent on signup")]
    UnverifiedAccount,
    #[error("Authentication Error")]
    Base64Error(#[from] B64Error),
    #[error("Token is either expired or invalid")]
    UuidError(#[from] uuid::Error)
    
    // #[error(transparent)]
    // UnexpectedError(#[from] anyhow::Error),
}

impl ApiError {
    /// Get the [`Status`] associated with the error
    fn status(&self) -> Status {
        use ApiError::*;

        match self {
            ValidationError(_) | BadRequest(_) => Status::BadRequest,
            DatabaseError(_) | PasswordError(_) | RedisError(_) | InternalServerError => Status::InternalServerError,
            JwtError(_) | UuidError(_) | AuthenticationError(_) | Base64Error(_) => Status::Unauthorized,
            Conflict(_) => Status::Conflict,
            AuthorizationError(_) | UnverifiedAccount => Status::Forbidden,
        }
    }
}


impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        let status = self.status();
        let error_response = ErrorResponse::Response {
            status: status.code,
            message: status.reason_lossy(),
            body: Some(self.to_string()),
        };

        let response = Response::build_from(Json(error_response).respond_to(request)?)
            .status(status)
            .finalize();

        Ok(response)
    }
}