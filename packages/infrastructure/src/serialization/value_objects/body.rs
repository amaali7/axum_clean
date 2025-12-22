use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Body;
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SerializedBody(String);

impl SerializedBody {
    pub fn new(body: &str) -> InfrastructureResult<Self> {
        let body = body.trim();

        if body.len() < 3 {
            return Err(InfrastructureError::ValidationError(
                "Body must be at least 3 characters".to_string(),
            ));
        }

        Ok(Self(body.to_string()))
    }
    pub fn body(&self) -> String {
        self.0.clone()
    }
}

impl Deref for SerializedBody {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializedBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for SerializedBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SerializedBody {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> InfrastructureResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Body> for SerializedBody {
    type Error = InfrastructureError;

    fn try_from(value: Body) -> InfrastructureResult<Self> {
        Self::new(&value.body())
    }
}

impl TryFrom<SerializedBody> for Body {
    type Error = InfrastructureError;

    fn try_from(value: SerializedBody) -> InfrastructureResult<Self> {
        Self::new(&value.body()).map_err(|err| InfrastructureError::Domain(err))
    }
}
