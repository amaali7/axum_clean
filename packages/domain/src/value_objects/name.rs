use std::ops::{Deref, DerefMut};

use crate::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Result<Self, DomainError> {
        let name = name.trim();

        if name.len() < 3 {
            return Err(DomainError::ValidationError(
                "Name must be at least 3 characters".to_string(),
            ));
        }

        if name.len() > 30 {
            return Err(DomainError::ValidationError(
                "Name must be less than 30 characters".to_string(),
            ));
        }

        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(DomainError::ValidationError(
                "Name can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }

        if name.starts_with('_') || name.starts_with('-') {
            return Err(DomainError::ValidationError(
                "Name cannot start with underscore or hyphen".to_string(),
            ));
        }

        Ok(Self(name.to_string()))
    }
}

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Name {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
