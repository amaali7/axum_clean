use std::ops::{Deref, DerefMut};

use domain::Username;
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SerializedUsername(String);

impl SerializedUsername {
    pub fn new(username: &str) -> InfrastructureResult<Self> {
        let username = username.trim();

        if username.len() < 3 {
            return Err(InfrastructureError::ValidationError(
                "Username must be at least 3 characters".to_string(),
            ));
        }

        if username.len() > 30 {
            return Err(InfrastructureError::ValidationError(
                "Username must be less than 30 characters".to_string(),
            ));
        }

        if !username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(InfrastructureError::ValidationError(
                "Username can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }

        if username.starts_with('_') || username.starts_with('-') {
            return Err(InfrastructureError::ValidationError(
                "Username cannot start with underscore or hyphen".to_string(),
            ));
        }

        Ok(Self(username.to_string()))
    }

    pub fn username(&self) -> String {
        self.0.clone()
    }
}

impl Deref for SerializedUsername {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializedUsername {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for SerializedUsername {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for SerializedUsername {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<Username> for SerializedUsername {
    type Error = InfrastructureError;

    fn try_from(value: Username) -> InfrastructureResult<Self> {
        Self::new(&value.username())
    }
}

impl TryFrom<SerializedUsername> for Username {
    type Error = InfrastructureError;

    fn try_from(value: SerializedUsername) -> InfrastructureResult<Self> {
        Self::new(&value.username()).map_err(|err| InfrastructureError::Domain(err))
    }
}
