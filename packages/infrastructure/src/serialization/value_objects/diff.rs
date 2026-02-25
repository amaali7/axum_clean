use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::value_objects::Diff;
use serde::{Deserialize, Serialize};

use crate::error::InfrastructureError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InfrastructureDiff(String);

impl InfrastructureDiff {
    pub fn new(diff: &str) -> Self {
        Self(diff.to_string())
    }
}

impl Deref for InfrastructureDiff {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InfrastructureDiff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InfrastructureDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InfrastructureDiff {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl From<Diff> for InfrastructureDiff {
    fn from(value: Diff) -> Self {
        Self::new(&value.diff())
    }
}

impl From<InfrastructureDiff> for Diff {
    fn from(value: InfrastructureDiff) -> Self {
        Self::new(&value.0)
    }
}
