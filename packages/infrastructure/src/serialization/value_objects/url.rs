use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use domain::Url;
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfrastructureUrl(String);

impl InfrastructureUrl {
    pub fn new(url: &str) -> InfrastructureResult<Self> {
        let url = url.trim();

        // Empty check
        if url.is_empty() {
            return Err(InfrastructureError::ValidationError(
                "InfrastructureURL cannot be empty".into(),
            ));
        }

        // Add default scheme if none present
        let raw = if url.contains("://") {
            Cow::Borrowed(url)
        } else {
            Cow::Owned(format!("https://{}", url))
        };

        Ok(Self(raw.into()))
    }

    pub fn url(&self) -> String {
        self.0.clone()
    }
}

impl Deref for InfrastructureUrl {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InfrastructureUrl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InfrastructureUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for InfrastructureUrl {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> InfrastructureResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Url> for InfrastructureUrl {
    type Error = InfrastructureError;

    fn try_from(value: Url) -> InfrastructureResult<Self> {
        Self::new(&value.url())
    }
}

impl TryFrom<InfrastructureUrl> for Url {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureUrl) -> InfrastructureResult<Self> {
        Self::new(&value.url()).map_err(|err| InfrastructureError::Domain(err))
    }
}
