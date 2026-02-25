use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::error::{InterfaceError, InterfaceResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InterfaceToken(String);

impl InterfaceToken {
    pub fn new(token: &str) -> Self {
        let token = token.trim();

        Self(token.to_string())
    }

    pub fn token(&self) -> String {
        self.0.clone()
    }
}

impl Deref for InterfaceToken {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfaceToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InterfaceToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InterfaceToken {
    type Err = InterfaceError;

    fn from_str(s: &str) -> InterfaceResult<Self> {
        Ok(Self::new(s))
    }
}
