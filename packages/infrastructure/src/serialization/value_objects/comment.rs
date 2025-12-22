use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Comment;
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SerializedComment(String);

impl SerializedComment {
    pub fn new(comment: &str) -> InfrastructureResult<Self> {
        let comment = comment.trim();

        if comment.len() < 3 {
            return Err(InfrastructureError::ValidationError(
                "SerializedComment must be at least 3 characters".to_string(),
            ));
        }

        Ok(Self(comment.to_string()))
    }
    pub fn comment(&self) -> String {
        self.0.clone()
    }
}

impl Deref for SerializedComment {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializedComment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for SerializedComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SerializedComment {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> InfrastructureResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Comment> for SerializedComment {
    type Error = InfrastructureError;

    fn try_from(value: Comment) -> InfrastructureResult<Self> {
        Self::new(&value.comment())
    }
}

impl TryFrom<SerializedComment> for Comment {
    type Error = InfrastructureError;

    fn try_from(value: SerializedComment) -> InfrastructureResult<Self> {
        Self::new(&value.comment()).map_err(|err| InfrastructureError::Domain(err))
    }
}
