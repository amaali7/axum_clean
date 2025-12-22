use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::value_objects::Language;
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SerializedLanguage(String);

impl SerializedLanguage {
    pub fn new(language: &str) -> InfrastructureResult<Self> {
        let language = language.trim();

        if language.len() < 3 {
            return Err(InfrastructureError::ValidationError(
                "Language must be at least 3 characters".to_string(),
            ));
        }

        Ok(Self(language.to_string()))
    }
    pub fn language(&self) -> String {
        self.0.clone()
    }
}

impl Deref for SerializedLanguage {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializedLanguage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for SerializedLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SerializedLanguage {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> InfrastructureResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Language> for SerializedLanguage {
    type Error = InfrastructureError;

    fn try_from(value: Language) -> InfrastructureResult<Self> {
        Self::new(&value.language())
    }
}

impl TryFrom<SerializedLanguage> for Language {
    type Error = InfrastructureError;

    fn try_from(value: SerializedLanguage) -> InfrastructureResult<Self> {
        Self::new(&value.language()).map_err(|err| InfrastructureError::Domain(err))
    }
}
