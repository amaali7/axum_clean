use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::{error::DomainResult, DomainError, SharedStr};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Conutry(SharedStr);

impl Conutry {
    pub fn new(conutry: &str) -> DomainResult<Self> {
        let conutry = conutry.trim();

        if conutry.len() < 3 {
            return Err(DomainError::ValidationError(
                "Conutry must be at least 3 characters".into(),
            ));
        }

        if conutry.len() > 60 {
            return Err(DomainError::ValidationError(
                "conutry must be less than 160 characters".into(),
            ));
        }

        Ok(Self(conutry.into()))
    }
    pub fn conutry(&self) -> &str {
        &self.0
    }
}

impl Deref for Conutry {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Conutry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Conutry {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}
