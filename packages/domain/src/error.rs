use thiserror::Error;

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
    ValidationError(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Domain invariant violation: {0}")]
    InvariantViolation(String),
}

#[derive(Error, Debug)]
pub enum RoleError {
    #[error("User not found")]
    NotFound,

    #[error("Invalid role ID: {0}")]
    InvalidRoleId(String),

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
    InvalidReportId(String),
}

#[derive(Error, Debug)]
pub enum PermissionError {
    #[error("User not found")]
    NotFound,

    #[error("Invalid user ID: {0}")]
    InvalidPermission(String),
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    NotFound,

    #[error("Invalid user ID: {0}")]
    InvalidUserId(String),

    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Invalid username: {0}")]
    InvalidUsername(String),

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
