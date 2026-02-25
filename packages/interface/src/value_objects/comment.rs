use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Comment;
use serde::{Deserialize, Serialize};

use crate::error::{InterfaceError, InterfaceResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterfaceComment(String);

impl InterfaceComment {
    pub fn new(comment: &str) -> InterfaceResult<Self> {
        let comment = comment.trim();

        if comment.len() < 3 {
            return Err(InterfaceError::ValidationError(
                "InterfaceComment must be at least 3 characters".to_string(),
            ));
        }

        Ok(Self(comment.to_string()))
    }
    pub fn comment(&self) -> String {
        self.0.clone()
    }
}

impl Deref for InterfaceComment {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfaceComment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InterfaceComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InterfaceComment {
    type Err = InterfaceError;

    fn from_str(s: &str) -> InterfaceResult<Self> {
        Self::new(s)
    }
}

impl TryFrom<Comment> for InterfaceComment {
    type Error = InterfaceError;

    fn try_from(value: Comment) -> InterfaceResult<Self> {
        Self::new(&value.comment())
    }
}

impl TryFrom<InterfaceComment> for Comment {
    type Error = InterfaceError;

    fn try_from(value: InterfaceComment) -> InterfaceResult<Self> {
        Self::new(&value.comment()).map_err(|err| InterfaceError::Domain(err))
    }
}
