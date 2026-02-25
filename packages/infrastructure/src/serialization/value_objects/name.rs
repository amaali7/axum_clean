use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Name;
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InfrastructureName(String);

impl InfrastructureName {
    pub fn new(name: &str) -> InfrastructureResult<Self> {
        let name = name.trim();

        if name.len() < 3 {
            return Err(InfrastructureError::ValidationError(
                "Name must be at least 3 characters".to_string(),
            ));
        }

        if name.len() > 30 {
            return Err(InfrastructureError::ValidationError(
                "Name must be less than 30 characters".to_string(),
            ));
        }

        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(InfrastructureError::ValidationError(
                "Name can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }

        if name.starts_with('_') || name.starts_with('-') {
            return Err(InfrastructureError::ValidationError(
                "Name cannot start with underscore or hyphen".to_string(),
            ));
        }

        Ok(Self(name.to_string()))
    }
    pub fn name(&self) -> String {
        self.0.clone()
    }
}

impl Deref for InfrastructureName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InfrastructureName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InfrastructureName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InfrastructureName {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<Name> for InfrastructureName {
    type Error = InfrastructureError;

    fn try_from(value: Name) -> InfrastructureResult<Self> {
        Self::new(&value.name())
    }
}

impl TryFrom<InfrastructureName> for Name {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureName) -> InfrastructureResult<Self> {
        Self::new(&value.name()).map_err(|err| InfrastructureError::Domain(err))
    }
}
