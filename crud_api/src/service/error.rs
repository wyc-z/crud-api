#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Empty: {0}")]
    Empty(String),
    #[error("{0}")]
    Conflict(String),
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Invalid: {0}")]
    Invalid(String),
    #[error("InternalError{0}")]
    InternalError(String),
}

impl From<AppError> for std::io::Error {
    fn from(err: AppError) -> Self {
        std::io::Error::other(err)
    }
}
