use jsonwebtoken::errors::Error as JwtError;
use thiserror::Error;
use rocket::{
    http::Status,
    response::{self, Responder},
    serde::json::Json,
    Request, Response,
};
use serde::{Deserialize, Serialize};


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
    #[error("Database Error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Jwt Error {0}")]
    JwtError(#[from] JwtError),
    #[error("{0}")]
    BadRequest(&'static str),
    #[error("{0}")]
    ValidationError(&'static str),
    #[error("{0}")]
    Conflict(&'static str),
    
    // #[error(transparent)]
    // UnexpectedError(#[from] anyhow::Error),
    // #[error("Username or Password is invalid")]
    // AuthenticationError(String),
    // #[error("Authorization Error")]
    // AuthorizationError(String),
}

impl ApiError {
    /// Get the [`Status`] associated with the error
    fn status(&self) -> Status {
        use ApiError::*;

        match self {
            ValidationError(_) | BadRequest(_) => Status::BadRequest,
            DatabaseError(_) => Status::InternalServerError,
            JwtError(_) => Status::Unauthorized,
            Conflict(_) => Status::Conflict,
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