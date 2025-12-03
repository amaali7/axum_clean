use std::ops::{Deref, DerefMut};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Title(String);

impl Title {
    pub fn new(title: &str) -> DomainResult<Self> {
        let title = title.trim();

        if title.len() < 3 {
            return Err(DomainError::ValidationError(
                "Title must be at least 3 characters".to_string(),
            ));
        }

        if title.len() > 30 {
            return Err(DomainError::ValidationError(
                "Title must be less than 30 characters".to_string(),
            ));
        }

        if !title
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(DomainError::ValidationError(
                "Title can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }

        if title.starts_with('_') || title.starts_with('-') {
            return Err(DomainError::ValidationError(
                "Title cannot start with underscore or hyphen".to_string(),
            ));
        }

        Ok(Self(title.to_string()))
    }
}

impl Deref for Title {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Title {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
