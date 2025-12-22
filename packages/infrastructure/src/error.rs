use application::error::ApplicationError;
use domain::error::DomainError;
// use surrealdb::error::{Api, Db};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Domain failed: {0}")]
    Domain(#[from] DomainError),

    #[error("Application  failed: {0}")]
    Application(#[from] ApplicationError),

    #[error("Repository failed: {0}")]
    Repository(String),

    // #[error("Surreal Api failed: {0}")]
    // SurrealApi(#[from] Api),
    #[error("Surreal Db failed: {0}")]
    Surreal(#[from] surrealdb::Error),

    #[error("Permission denied")]
    Forbidden,

    #[error("Infrastucter Validation failed: {0}")]
    ValidationError(String),
    #[error("Pasword unhashed error")]
    UnHashedPassword,

    #[error("Invalid timestamp failed")]
    InvalidTimestamp,
    #[error("Unknown application error")]
    Unknown,
}

pub type InfrastructureResult<T> = Result<T, InfrastructureError>;
