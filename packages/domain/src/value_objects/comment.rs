use std::ops::{Deref, DerefMut};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone, Default)]
pub struct Comment(String);

impl Comment {
    pub fn new(comment: &str) -> DomainResult<Self> {
        let comment = comment.trim();

        if comment.len() < 3 {
            return Err(DomainError::ValidationError(
                "Comment must be at least 3 characters".to_string(),
            ));
        }

        Ok(Self(comment.to_string()))
    }
}

impl Deref for Comment {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Comment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
