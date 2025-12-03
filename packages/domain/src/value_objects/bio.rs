use std::ops::{Deref, DerefMut};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone, Default)]
pub struct Bio(String);

impl Bio {
    pub fn new(bio: &str) -> DomainResult<Self> {
        let bio = bio.trim();

        if bio.len() < 3 {
            return Err(DomainError::ValidationError(
                "Bio must be at least 3 characters".to_string(),
            ));
        }

        if bio.len() > 160 {
            return Err(DomainError::ValidationError(
                "bio must be less than 160 characters".to_string(),
            ));
        }

        Ok(Self(bio.to_string()))
    }
}

impl Deref for Bio {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Bio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
