use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Description;
use serde::{Deserialize, Serialize};

use crate::error::{InterfaceError, InterfaceResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InterfaceDescription(String);

impl InterfaceDescription {
    pub fn new(description: &str) -> InterfaceResult<Self> {
        let description = description.trim();

        if description.len() < 3 {
            return Err(InterfaceError::ValidationError(
                "Description must be at least 3 characters".to_string(),
            ));
        }
        Ok(Self(description.to_string()))
    }
    pub fn description(&self) -> String {
        self.0.clone()
    }
}

impl Deref for InterfaceDescription {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfaceDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InterfaceDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InterfaceDescription {
    type Err = InterfaceError;

    fn from_str(s: &str) -> InterfaceResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Description> for InterfaceDescription {
    type Error = InterfaceError;

    fn try_from(value: Description) -> InterfaceResult<Self> {
        Self::new(&value.description())
    }
}

impl TryFrom<InterfaceDescription> for Description {
    type Error = InterfaceError;

    fn try_from(value: InterfaceDescription) -> InterfaceResult<Self> {
        Self::new(&value.description()).map_err(|err| InterfaceError::Domain(err))
    }
}
