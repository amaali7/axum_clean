use std::ops::{Deref, DerefMut};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone, Default)]
pub struct DateTime(i64);

impl DateTime {
    pub fn new(timestamp: i64) -> DomainResult<Self> {
        // Validate that it's a reasonable timestamp (between 1970 and 2100)
        if timestamp < 0 {
            return Err(DomainError::ValidationError(
                "Timestamp cannot be negative".to_string(),
            ));
        }

        // Check if it's a reasonable future timestamp (up to year 2100)
        let max_reasonable = 4102444800; // 2100-01-01
        if timestamp > max_reasonable {
            return Err(DomainError::ValidationError(
                "Timestamp is too far in the future".to_string(),
            ));
        }

        Ok(Self(timestamp))
    }
}

impl Deref for DateTime {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
