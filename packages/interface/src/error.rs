use application::error::AppError;
use domain::error::DomainError;
// use surrealdb::error::{Api, Db};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterfaceError {
    #[error("Domain failed: {0}")]
    Domain(#[from] DomainError),

    #[error("Application  failed: {0}")]
    Application(#[from] AppError),

    #[error("Repository failed: {0}")]
    Http(String),

    #[error("Permission denied")]
    Forbidden,
    #[error("Infrastucter Validation failed: {0}")]
    ValidationError(String),
    #[error("Pasword unhashed error")]
    UnHashedPassword,

    #[error("Invalid timestamp failed")]
    InvalidTimestamp,
    #[error("Unknown application error: {0}")]
    Unknown(String),
}

pub type InterfaceResult<T> = Result<T, InterfaceError>;

// For converting InterfaceError to DomainError
impl From<InterfaceError> for DomainError {
    fn from(error: InterfaceError) -> Self {
        match error {
            InterfaceError::Domain(domain_error) => domain_error,
            InterfaceError::Application(application_error) => {
                DomainError::InvalidOperation(application_error.to_string())
            }
            InterfaceError::Http(repo_error) => {
                DomainError::InvalidOperation(repo_error.to_string())
            }
            InterfaceError::Forbidden => {
                DomainError::InvalidOperation("Forbidden Operation".to_string())
            }
            InterfaceError::ValidationError(validation_error) => {
                DomainError::ValidationError(validation_error.to_string())
            }
            InterfaceError::UnHashedPassword => {
                DomainError::ValidationError("Unhashed Password".to_string())
            }
            InterfaceError::InvalidTimestamp => {
                DomainError::ValidationError("Invalid DateTime".to_string())
            }
            InterfaceError::Unknown(unknown) => DomainError::InvalidOperation(unknown),
        }
    }
}

// For converting InterfaceError to AppError
impl From<InterfaceError> for AppError {
    fn from(error: InterfaceError) -> Self {
        match error {
            InterfaceError::Domain(domain_error) => AppError::Domain(domain_error),
            InterfaceError::Application(application_error) => application_error,
            InterfaceError::Http(repo_error) => {
                AppError::Repository(repo_error.to_string())
            }
            InterfaceError::Forbidden => AppError::Forbidden,
            InterfaceError::ValidationError(validation_error) => {
                AppError::ValidationError(validation_error)
            }
            InterfaceError::UnHashedPassword => {
                AppError::ValidationError("UnHashed Password".to_string())
            }
            InterfaceError::InvalidTimestamp => {
                AppError::ValidationError("Invalid DateTime".to_string())
            }
            InterfaceError::Unknown(un) => AppError::Unknown(un),
        }
    }
}
