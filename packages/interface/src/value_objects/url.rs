use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use domain::Url;
use serde::{Deserialize, Serialize};

use crate::error::{InterfaceError, InterfaceResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InterfaceUrl(String);

impl InterfaceUrl {
    pub fn new(url: &str) -> InterfaceResult<Self> {
        let url = url.trim();

        // Empty check
        if url.is_empty() {
            return Err(InterfaceError::ValidationError(
                "InterfaceURL cannot be empty".into(),
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

impl Deref for InterfaceUrl {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfaceUrl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InterfaceUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for InterfaceUrl {
    type Err = InterfaceError;

    fn from_str(s: &str) -> InterfaceResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Url> for InterfaceUrl {
    type Error = InterfaceError;

    fn try_from(value: Url) -> InterfaceResult<Self> {
        Self::new(&value.url())
    }
}

impl TryFrom<InterfaceUrl> for Url {
    type Error = InterfaceError;

    fn try_from(value: InterfaceUrl) -> InterfaceResult<Self> {
        Self::new(&value.url()).map_err(|err| InterfaceError::Domain(err))
    }
}
