use std::{fmt, str::FromStr};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone)]
pub enum Password {
    Hashed(HashedPassword),

    NoneHashed(NoneHashedPassword),
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Password::Hashed(hashed_password) => write!(f, "{}", hashed_password),
            Password::NoneHashed(none_hashed_password) => write!(f, "{}", none_hashed_password),
        }
    }
}

impl Default for Password {
    fn default() -> Self {
        Password::NoneHashed(NoneHashedPassword::default())
    }
}

#[derive(Debug, Default, Clone)]
pub struct NoneHashedPassword(String);

impl fmt::Display for NoneHashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl NoneHashedPassword {
    pub fn new(password: &str) -> DomainResult<Self> {
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

impl FromStr for NoneHashedPassword {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s)?)
    }
}

#[derive(Debug, Default, Clone)]
pub struct HashedPassword(String);

impl fmt::Display for HashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl HashedPassword {
    pub fn new(hashed_password: &str) -> DomainResult<Self> {
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
