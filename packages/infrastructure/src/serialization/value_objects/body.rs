use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Body;
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfrastructureBody(String);

impl InfrastructureBody {
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

impl Deref for InfrastructureBody {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InfrastructureBody {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InfrastructureBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InfrastructureBody {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> InfrastructureResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Body> for InfrastructureBody {
    type Error = InfrastructureError;

    fn try_from(value: Body) -> InfrastructureResult<Self> {
        Self::new(&value.body())
    }
}

impl TryFrom<InfrastructureBody> for Body {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureBody) -> InfrastructureResult<Self> {
        Self::new(&value.body()).map_err(|err| InfrastructureError::Domain(err))
    }
}
