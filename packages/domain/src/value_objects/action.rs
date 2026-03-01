use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Action(String);

impl Action {
    pub fn new(action: &str) -> DomainResult<Self> {
        let action = action.trim();

        if action.len() < 3 {
            return Err(DomainError::ValidationError(
                "Action must be at least 3 characters".to_string(),
            ));
        }

        if action.len() > 30 {
            return Err(DomainError::ValidationError(
                "Action must be less than 30 characters".to_string(),
            ));
        }

        if !action
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(DomainError::ValidationError(
                "Action can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }

        if action.starts_with('_') || action.starts_with('-') {
            return Err(DomainError::ValidationError(
                "Action cannot start with underscore or hyphen".to_string(),
            ));
        }

        Ok(Self(action.to_string()))
    }

    pub fn action(&self) -> String {
        self.0.clone()
    }
}

impl Deref for Action {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Action {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Action {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}
