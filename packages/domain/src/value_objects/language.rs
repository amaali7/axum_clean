use std::ops::{Deref, DerefMut};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone, Default)]
pub struct Language(String);

impl Language {
    pub fn new(language: &str) -> DomainResult<Self> {
        let language = language.trim();

        if language.len() < 3 {
            return Err(DomainError::ValidationError(
                "Language must be at least 3 characters".to_string(),
            ));
        }

        Ok(Self(language.to_string()))
    }
}

impl Deref for Language {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Language {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
