use std::ops::{Deref, DerefMut};

use domain::Title;
use serde::{Deserialize, Serialize};

use crate::error::{InterfaceError, InterfaceResult};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InterfaceTitle(String);

impl InterfaceTitle {
    pub fn new(title: &str) -> InterfaceResult<Self> {
        let title = title.trim();

        if title.len() < 3 {
            return Err(InterfaceError::ValidationError(
                "Title must be at least 3 characters".to_string(),
            ));
        }

        if title.len() > 30 {
            return Err(InterfaceError::ValidationError(
                "Title must be less than 30 characters".to_string(),
            ));
        }

        if !title
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(InterfaceError::ValidationError(
                "Title can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }

        if title.starts_with('_') || title.starts_with('-') {
            return Err(InterfaceError::ValidationError(
                "Title cannot start with underscore or hyphen".to_string(),
            ));
        }

        Ok(Self(title.to_string()))
    }
    pub fn title(&self) -> String {
        self.0.clone()
    }
}

impl Deref for InterfaceTitle {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfaceTitle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InterfaceTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for InterfaceTitle {
    type Err = InterfaceError;

    fn from_str(s: &str) -> InterfaceResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Title> for InterfaceTitle {
    type Error = InterfaceError;

    fn try_from(value: Title) -> InterfaceResult<Self> {
        Self::new(&value.title())
    }
}

impl TryFrom<InterfaceTitle> for Title {
    type Error = InterfaceError;

    fn try_from(value: InterfaceTitle) -> InterfaceResult<Self> {
        Self::new(&value.title()).map_err(|err| InterfaceError::Domain(err))
    }
}
