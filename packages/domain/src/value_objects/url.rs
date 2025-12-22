use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone, Default)]
pub struct Url(String);

impl Url {
    pub fn new(url: &str) -> DomainResult<Self> {
        let url = url.trim();

        // Empty check
        if url.is_empty() {
            return Err(DomainError::ValidationError("URL cannot be empty".into()));
        }

        // Add default scheme if none present
        let raw = if url.contains("://") {
            Cow::Borrowed(url)
        } else {
            Cow::Owned(format!("https://{}", url))
        };

        Ok(Self(raw.into()))
    }

    pub fn url(&self) -> String {
        self.0.clone()
    }
}

impl Deref for Url {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Url {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Url {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}
