use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Diff(String);

impl Diff {
    pub fn new(diff: &str) -> Self {
        Self(diff.to_string())
    }
    pub fn diff(&self) -> String {
        self.0.clone()
    }
}

impl Deref for Diff {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Diff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Diff {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
