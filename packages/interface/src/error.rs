use application::error::ApplicationError;
use domain::error::DomainError;
// use surrealdb::error::{Api, Db};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterfaceError {
    #[error("Domain failed: {0}")]
    Domain(#[from] DomainError),

    #[error("Application  failed: {0}")]
    Application(#[from] ApplicationError),

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

// For converting InterfaceError to ApplicationError
impl From<InterfaceError> for ApplicationError {
    fn from(error: InterfaceError) -> Self {
        match error {
            InterfaceError::Domain(domain_error) => ApplicationError::Domain(domain_error),
            InterfaceError::Application(application_error) => application_error,
            InterfaceError::Http(repo_error) => {
                ApplicationError::Repository(repo_error.to_string())
            }
            InterfaceError::Forbidden => ApplicationError::Forbidden,
            InterfaceError::ValidationError(validation_error) => {
                ApplicationError::ValidationError(validation_error)
            }
            InterfaceError::UnHashedPassword => {
                ApplicationError::ValidationError("UnHashed Password".to_string())
            }
            InterfaceError::InvalidTimestamp => {
                ApplicationError::ValidationError("Invalid DateTime".to_string())
            }
            InterfaceError::Unknown(un) => ApplicationError::Unknown(un),
        }
    }
}
