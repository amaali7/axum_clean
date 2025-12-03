use domain::error::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain validation failed: {0}")]
    Domain(#[from] DomainError),

    #[error("Repository failed: {0}")]
    Repository(String),

    #[error("Permission denied")]
    Forbidden,

    #[error("Unknown application error")]
    Unknown,
}

pub type AppResult<T> = Result<T, ApplicationError>;
