use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfrastructureToken(String);

impl InfrastructureToken {
    pub fn new(token: &str) -> Self {
        let token = token.trim();

        Self(token.to_string())
    }

    pub fn token(&self) -> String {
        self.0.clone()
    }
}

impl Deref for InfrastructureToken {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InfrastructureToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InfrastructureToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InfrastructureToken {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> InfrastructureResult<Self> {
        Ok(Self::new(s))
    }
}
