use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Bio;
use serde::{Deserialize, Serialize};

use crate::error::{InterfaceError, InterfaceResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InterfaceBio(String);

impl InterfaceBio {
    pub fn new(bio: &str) -> InterfaceResult<Self> {
        let bio = bio.trim();

        if bio.len() < 3 {
            return Err(InterfaceError::ValidationError(
                "Bio must be at least 3 characters".to_string(),
            ));
        }

        if bio.len() > 160 {
            return Err(InterfaceError::ValidationError(
                "bio must be less than 160 characters".to_string(),
            ));
        }

        Ok(Self(bio.to_string()))
    }

    pub fn bio(&self) -> String {
        self.0.clone()
    }
}

impl Deref for InterfaceBio {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfaceBio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InterfaceBio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InterfaceBio {
    type Err = InterfaceError;

    fn from_str(s: &str) -> InterfaceResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<InterfaceBio> for Bio {
    type Error = InterfaceError;

    fn try_from(value: InterfaceBio) -> InterfaceResult<Self> {
        Self::new(&value.bio()).map_err(|err| InterfaceError::Domain(err))
    }
}

impl TryFrom<Bio> for InterfaceBio {
    type Error = InterfaceError;

    fn try_from(value: Bio) -> InterfaceResult<Self> {
        Self::new(&value.bio())
    }
}
