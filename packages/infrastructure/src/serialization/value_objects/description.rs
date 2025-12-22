use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Description;
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SerializedDescription(String);

impl SerializedDescription {
    pub fn new(description: &str) -> InfrastructureResult<Self> {
        let description = description.trim();

        if description.len() < 3 {
            return Err(InfrastructureError::ValidationError(
                "Description must be at least 3 characters".to_string(),
            ));
        }
        Ok(Self(description.to_string()))
    }
    pub fn description(&self) -> String {
        self.0.clone()
    }
}

impl Deref for SerializedDescription {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializedDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for SerializedDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SerializedDescription {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> InfrastructureResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Description> for SerializedDescription {
    type Error = InfrastructureError;

    fn try_from(value: Description) -> InfrastructureResult<Self> {
        Self::new(&value.description())
    }
}

impl TryFrom<SerializedDescription> for Description {
    type Error = InfrastructureError;

    fn try_from(value: SerializedDescription) -> InfrastructureResult<Self> {
        Self::new(&value.description()).map_err(|err| InfrastructureError::Domain(err))
    }
}
