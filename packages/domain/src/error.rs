use thiserror::Error;

use crate::SharedStr;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("User error: {0}")]
    UserError(#[from] UserError),

    #[error("Role error: {0}")]
    RoleError(#[from] RoleError),

    #[error("Report error: {0}")]
    ReportError(#[from] ReportError),

    #[error("Permission error: {0}")]
    PermissionError(#[from] PermissionError),

    #[error("Validation error: {0}")]
    ValidationError(SharedStr),

    #[error("Invalid operation: {0}")]
    InvalidOperation(SharedStr),

    #[error("Domain invariant violation: {0}")]
    InvariantViolation(SharedStr),
}

#[derive(Error, Debug)]
pub enum RoleError {
    #[error("User not found")]
    NotFound,

    #[error("Invalid role ID: {0}")]
    InvalidRoleId(SharedStr),

    #[error("Permission: {0}")]
    PermissionError(#[from] PermissionError),
}

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Permission error: {0}")]
    PermissionError(#[from] PermissionError),
    #[error("Body empty")]
    BodyEmpty,
    #[error("Report not found")]
    NotFound,

    #[error("Invalid report ID: {0}")]
    InvalidReportId(SharedStr),
}

#[derive(Error, Debug)]
pub enum PermissionError {
    #[error("User not found")]
    NotFound,

    #[error("Invalid user ID: {0}")]
    InvalidPermission(SharedStr),
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    NotFound,

    #[error("Invalid user ID: {0}")]
    InvalidUserId(SharedStr),

    #[error("Invalid email: {0}")]
    InvalidEmail(SharedStr),

    #[error("Invalid username: {0}")]
    InvalidUsername(SharedStr),

    #[error("Invalid password")]
    InvalidPassword,

    #[error("User is suspended")]
    Suspended,

    #[error("User is not active")]
    NotActive,

    #[error("Insufficient permissions")]
    InsufficientPermissions,
}

pub type DomainResult<T> = Result<T, DomainError>;
