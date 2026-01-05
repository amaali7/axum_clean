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

// For converting InfrastructureError to DomainError
impl From<InfrastructureError> for DomainError {
    fn from(error: InfrastructureError) -> Self {
        match error {
            InfrastructureError::Domain(domain_error) => domain_error,
            InfrastructureError::Application(application_error) => {
                DomainError::InvalidOperation(application_error.to_string())
            }
            InfrastructureError::Repository(repo_error) => {
                DomainError::InvalidOperation(repo_error.to_string())
            }
            InfrastructureError::Surreal(error) => DomainError::InvalidOperation(error.to_string()),
            InfrastructureError::Forbidden => {
                DomainError::InvalidOperation("Forbidden Operation".to_string())
            }
            InfrastructureError::ValidationError(validation_error) => {
                DomainError::ValidationError(validation_error.to_string())
            }
            InfrastructureError::UnHashedPassword => {
                DomainError::ValidationError("Unhashed Password".to_string())
            }
            InfrastructureError::InvalidTimestamp => {
                DomainError::ValidationError("Invalid DateTime".to_string())
            }
            InfrastructureError::Unknown => {
                DomainError::InvalidOperation("UnKnown Error".to_string())
            }
        }
    }
}

// For converting InfrastructureError to ApplicationError
impl From<InfrastructureError> for ApplicationError {
    fn from(error: InfrastructureError) -> Self {
        match error {
            InfrastructureError::Domain(domain_error) => ApplicationError::Domain(domain_error),
            InfrastructureError::Application(application_error) => application_error,
            InfrastructureError::Repository(repo_error) => {
                ApplicationError::Repository(repo_error.to_string())
            }
            InfrastructureError::Surreal(error) => ApplicationError::Repository(error.to_string()),
            InfrastructureError::Forbidden => ApplicationError::Forbidden,
            InfrastructureError::ValidationError(validation_error) => {
                ApplicationError::ValidationError(validation_error)
            }
            InfrastructureError::UnHashedPassword => {
                ApplicationError::ValidationError("UnHashed Password".to_string())
            }
            InfrastructureError::InvalidTimestamp => {
                ApplicationError::ValidationError("Invalid DateTime".to_string())
            }
            InfrastructureError::Unknown => ApplicationError::Unknown,
        }
    }
}
