use std::ops::{Deref, DerefMut};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone, Default)]
pub struct Description(String);

impl Description {
    pub fn new(description: &str) -> DomainResult<Self> {
        let description = description.trim();

        if description.len() < 3 {
            return Err(DomainError::ValidationError(
                "Description must be at least 3 characters".to_string(),
            ));
        }
        Ok(Self(description.to_string()))
    }
}

impl Deref for Description {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Description {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
