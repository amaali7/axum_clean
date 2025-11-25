use std::ops::{Deref, DerefMut};

use crate::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

impl Username {
    pub fn new(username: &str) -> Result<Self, DomainError> {
        let username = username.trim();

        if username.len() < 3 {
            return Err(DomainError::ValidationError(
                "Username must be at least 3 characters".to_string(),
            ));
        }

        if username.len() > 30 {
            return Err(DomainError::ValidationError(
                "Username must be less than 30 characters".to_string(),
            ));
        }

        if !username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(DomainError::ValidationError(
                "Username can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }

        if username.starts_with('_') || username.starts_with('-') {
            return Err(DomainError::ValidationError(
                "Username cannot start with underscore or hyphen".to_string(),
            ));
        }

        Ok(Self(username.to_string()))
    }
}

impl Deref for Username {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Username {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
