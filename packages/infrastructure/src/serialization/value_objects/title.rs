use std::ops::{Deref, DerefMut};

use domain::Title;
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializedTitle(String);

impl SerializedTitle {
    pub fn new(title: &str) -> InfrastructureResult<Self> {
        let title = title.trim();

        if title.len() < 3 {
            return Err(InfrastructureError::ValidationError(
                "Title must be at least 3 characters".to_string(),
            ));
        }

        if title.len() > 30 {
            return Err(InfrastructureError::ValidationError(
                "Title must be less than 30 characters".to_string(),
            ));
        }

        if !title
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(InfrastructureError::ValidationError(
                "Title can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }

        if title.starts_with('_') || title.starts_with('-') {
            return Err(InfrastructureError::ValidationError(
                "Title cannot start with underscore or hyphen".to_string(),
            ));
        }

        Ok(Self(title.to_string()))
    }
    pub fn title(&self) -> String {
        self.0.clone()
    }
}

impl Deref for SerializedTitle {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializedTitle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for SerializedTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for SerializedTitle {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> InfrastructureResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Title> for SerializedTitle {
    type Error = InfrastructureError;

    fn try_from(value: Title) -> InfrastructureResult<Self> {
        Self::new(&value.title())
    }
}

impl TryFrom<SerializedTitle> for Title {
    type Error = InfrastructureError;

    fn try_from(value: SerializedTitle) -> InfrastructureResult<Self> {
        Self::new(&value.title()).map_err(|err| InfrastructureError::Domain(err))
    }
}
