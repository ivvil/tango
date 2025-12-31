use thiserror::Error;

#[derive(Debug, Error)]
pub enum TangoError {
    #[error("Database error")]
    Db(#[from] sqlx::Error),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden
}