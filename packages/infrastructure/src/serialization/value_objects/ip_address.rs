use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SerializedIpAddress(String);

impl SerializedIpAddress {
    pub fn new(ip_address: &str) -> Self {
        let ip_address = ip_address.trim();

        Self(ip_address.to_string())
    }

    pub fn ip_address(&self) -> String {
        self.0.clone()
    }
}

impl Deref for SerializedIpAddress {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializedIpAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for SerializedIpAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SerializedIpAddress {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> InfrastructureResult<Self> {
        Ok(Self::new(s))
    }
}
