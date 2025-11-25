use std::str::FromStr;

use crate::DomainError;

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(password: &str) -> Result<Self, DomainError> {
        if password.len() < 8 {
            return Err(DomainError::ValidationError(
                "Password must be at least 8 characters".to_string(),
            ));
        }

        if password.len() > 100 {
            return Err(DomainError::ValidationError(
                "Password too long".to_string(),
            ));
        }

        // Check for common passwords in real implementation
        Ok(Self(password.to_string()))
    }

    // Note: No Deref implementation for security - don't expose password
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Password {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s)?)
    }
}

#[derive(Debug, Clone)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn new(hashed_password: &str) -> Result<Self, DomainError> {
        if hashed_password.len() < 43 || hashed_password.len() > 128 {
            return Err(DomainError::ValidationError(
                "Its not Hashed Password".to_string(),
            ));
        }

        // Check for common passwords in real implementation
        Ok(Self(hashed_password.to_string()))
    }

    // Note: No Deref implementation for security - don't expose hashed_password
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
