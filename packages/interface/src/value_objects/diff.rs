use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::value_objects::Diff;
use serde::{Deserialize, Serialize};

use crate::error::InterfaceError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InterfaceDiff(String);

impl InterfaceDiff {
    pub fn new(diff: &str) -> Self {
        Self(diff.to_string())
    }
}

impl Deref for InterfaceDiff {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfaceDiff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InterfaceDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InterfaceDiff {
    type Err = InterfaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl From<Diff> for InterfaceDiff {
    fn from(value: Diff) -> Self {
        Self::new(&value.diff())
    }
}

impl From<InterfaceDiff> for Diff {
    fn from(value: InterfaceDiff) -> Self {
        Self::new(&value.0)
    }
}
