use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Bio;
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SerializedBio(String);

impl SerializedBio {
    pub fn new(bio: &str) -> InfrastructureResult<Self> {
        let bio = bio.trim();

        if bio.len() < 3 {
            return Err(InfrastructureError::ValidationError(
                "Bio must be at least 3 characters".to_string(),
            ));
        }

        if bio.len() > 160 {
            return Err(InfrastructureError::ValidationError(
                "bio must be less than 160 characters".to_string(),
            ));
        }

        Ok(Self(bio.to_string()))
    }

    pub fn bio(&self) -> String {
        self.0.clone()
    }
}

impl Deref for SerializedBio {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializedBio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for SerializedBio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SerializedBio {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> InfrastructureResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<SerializedBio> for Bio {
    type Error = InfrastructureError;

    fn try_from(value: SerializedBio) -> InfrastructureResult<Self> {
        Self::new(&value.bio()).map_err(|err| InfrastructureError::Domain(err))
    }
}

impl TryFrom<Bio> for SerializedBio {
    type Error = InfrastructureError;

    fn try_from(value: Bio) -> InfrastructureResult<Self> {
        Self::new(&value.bio())
    }
}
