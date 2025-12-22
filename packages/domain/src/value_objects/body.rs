use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone, Default)]
pub struct Body(String);

impl Body {
    pub fn new(body: &str) -> DomainResult<Self> {
        let body = body.trim();

        if body.len() < 3 {
            return Err(DomainError::ValidationError(
                "Body must be at least 3 characters".to_string(),
            ));
        }

        Ok(Self(body.to_string()))
    }
    pub fn body(&self) -> String {
        self.0.clone()
    }
}

impl Deref for Body {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Body {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Body {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}
