use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

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
    pub fn language(&self) -> String {
        self.0.clone()
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

impl FromStr for Language {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}
