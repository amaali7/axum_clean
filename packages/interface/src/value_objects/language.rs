use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::value_objects::Language;
use serde::{Deserialize, Serialize};

use crate::error::{InterfaceError, InterfaceResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InterfaceLanguage(String);

impl InterfaceLanguage {
    pub fn new(language: &str) -> InterfaceResult<Self> {
        let language = language.trim();

        if language.len() < 3 {
            return Err(InterfaceError::ValidationError(
                "Language must be at least 3 characters".to_string(),
            ));
        }

        Ok(Self(language.to_string()))
    }
    pub fn language(&self) -> String {
        self.0.clone()
    }
}

impl Deref for InterfaceLanguage {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfaceLanguage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InterfaceLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InterfaceLanguage {
    type Err = InterfaceError;

    fn from_str(s: &str) -> InterfaceResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Language> for InterfaceLanguage {
    type Error = InterfaceError;

    fn try_from(value: Language) -> InterfaceResult<Self> {
        Self::new(&value.language())
    }
}

impl TryFrom<InterfaceLanguage> for Language {
    type Error = InterfaceError;

    fn try_from(value: InterfaceLanguage) -> InterfaceResult<Self> {
        Self::new(&value.language()).map_err(|err| InterfaceError::Domain(err))
    }
}
