use std::{ops::Deref, str::FromStr};

use crate::{error::DomainResult, DomainError, SharedStr};

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub struct Description(SharedStr);

impl Description {
    pub fn new(description: &str) -> DomainResult<Self> {
        let description = description.trim();

        if description.len() < 3 {
            return Err(DomainError::ValidationError(
                "Description must be at least 3 characters".into(),
            ));
        }
        Ok(Self(description.into()))
    }

    pub fn description(&self) -> &str {
        &self.0
    }
}

impl Deref for Description {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Description {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}
