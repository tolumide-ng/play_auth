

#[derive(Debug, thiserror::Error)]
pub enum TError {
    #[error("Database Error")]
    DatabaseError(#[from] sqlx::Error),
}