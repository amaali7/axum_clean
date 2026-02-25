use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Name;
use serde::{Deserialize, Serialize};

use crate::error::{InterfaceError, InterfaceResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InterfaceName(String);

impl InterfaceName {
    pub fn new(name: &str) -> InterfaceResult<Self> {
        let name = name.trim();

        if name.len() < 3 {
            return Err(InterfaceError::ValidationError(
                "Name must be at least 3 characters".to_string(),
            ));
        }

        if name.len() > 30 {
            return Err(InterfaceError::ValidationError(
                "Name must be less than 30 characters".to_string(),
            ));
        }

        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(InterfaceError::ValidationError(
                "Name can only contain alphanumeric characters, underscores, and hyphens"
                    .to_string(),
            ));
        }

        if name.starts_with('_') || name.starts_with('-') {
            return Err(InterfaceError::ValidationError(
                "Name cannot start with underscore or hyphen".to_string(),
            ));
        }

        Ok(Self(name.to_string()))
    }
    pub fn name(&self) -> String {
        self.0.clone()
    }
}

impl Deref for InterfaceName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfaceName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InterfaceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InterfaceName {
    type Err = InterfaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<Name> for InterfaceName {
    type Error = InterfaceError;

    fn try_from(value: Name) -> InterfaceResult<Self> {
        Self::new(&value.name())
    }
}

impl TryFrom<InterfaceName> for Name {
    type Error = InterfaceError;

    fn try_from(value: InterfaceName) -> InterfaceResult<Self> {
        Self::new(&value.name()).map_err(|err| InterfaceError::Domain(err))
    }
}
