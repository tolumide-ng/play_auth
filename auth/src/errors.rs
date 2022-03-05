use jsonwebtoken::errors::Error as JwtError;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum TError {
    #[error("Database Error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Jwt Error {0}")]
    JwtError(#[from] JwtError),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    // #[error("Username or Password is invalid")]
    // AuthenticationError(String),
    // #[error("Authorization Error")]
    // AuthorizationError(String),
}
