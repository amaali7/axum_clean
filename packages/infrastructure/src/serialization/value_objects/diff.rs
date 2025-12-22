use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::value_objects::Diff;
use serde::{Deserialize, Serialize};

use crate::error::InfrastructureError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SerializedDiff(String);

impl SerializedDiff {
    pub fn new(diff: &str) -> Self {
        Self(diff.to_string())
    }
}

impl Deref for SerializedDiff {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializedDiff {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for SerializedDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SerializedDiff {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl From<Diff> for SerializedDiff {
    fn from(value: Diff) -> Self {
        Self::new(&value.diff())
    }
}

impl From<SerializedDiff> for Diff {
    fn from(value: SerializedDiff) -> Self {
        Self::new(&value.0)
    }
}
