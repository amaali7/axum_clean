use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Body;
use serde::{Deserialize, Serialize};

use crate::error::{InterfaceError, InterfaceResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InterfaceBody(String);

impl InterfaceBody {
    pub fn new(body: &str) -> InterfaceResult<Self> {
        let body = body.trim();

        if body.len() < 3 {
            return Err(InterfaceError::ValidationError(
                "Body must be at least 3 characters".to_string(),
            ));
        }

        Ok(Self(body.to_string()))
    }
    pub fn body(&self) -> String {
        self.0.clone()
    }
}

impl Deref for InterfaceBody {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfaceBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InterfaceBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InterfaceBody {
    type Err = InterfaceError;

    fn from_str(s: &str) -> InterfaceResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Body> for InterfaceBody {
    type Error = InterfaceError;

    fn try_from(value: Body) -> InterfaceResult<Self> {
        Self::new(&value.body())
    }
}

impl TryFrom<InterfaceBody> for Body {
    type Error = InterfaceError;

    fn try_from(value: InterfaceBody) -> InterfaceResult<Self> {
        Self::new(&value.body()).map_err(|err| InterfaceError::Domain(err))
    }
}
