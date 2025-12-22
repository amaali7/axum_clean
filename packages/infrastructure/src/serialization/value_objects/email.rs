use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Email;
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializedEmail(String);

impl SerializedEmail {
    pub fn new(email: &str) -> InfrastructureResult<Self> {
        let email = email.trim().to_lowercase();

        if email.len() > 254 {
            return Err(InfrastructureError::ValidationError(
                "Email too long".to_string(),
            ));
        }

        if !email.contains('@') {
            return Err(InfrastructureError::ValidationError(
                "Email must contain @".to_string(),
            ));
        }

        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(InfrastructureError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }

        if !parts[1].contains('.') {
            return Err(InfrastructureError::ValidationError(
                "Invalid Infrastructuer in email".to_string(),
            ));
        }

        Ok(Self(email))
    }

    pub fn email(&self) -> String {
        self.0.clone()
    }

    // pub fn domain(&self) -> &str {
    //     self.0.split('@').nth(1).unwrap_or("")
    // }

    // pub fn is_disposable(&self) -> bool {
    //     let disposable_domains = ["tempmail.com", "throwaway.com", "guerrillamail.com"];
    //     disposable_domains
    //         .iter()
    //         .any(|domain| self.domain() == *domain)
    // }
}

impl Deref for SerializedEmail {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializedEmail {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for SerializedEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for SerializedEmail {
    type Err = InfrastructureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<Email> for SerializedEmail {
    type Error = InfrastructureError;

    fn try_from(value: Email) -> InfrastructureResult<Self> {
        Self::new(&value.email())
    }
}

impl TryFrom<SerializedEmail> for Email {
    type Error = InfrastructureError;

    fn try_from(value: SerializedEmail) -> InfrastructureResult<Self> {
        Self::new(&value.email()).map_err(|err| InfrastructureError::Domain(err))
    }
}
