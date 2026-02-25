use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};

use domain::Email;
use serde::{Deserialize, Serialize};

use crate::error::{InterfaceError, InterfaceResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InterfaceEmail(String);

impl InterfaceEmail {
    pub fn new(email: &str) -> InterfaceResult<Self> {
        let email = email.trim().to_lowercase();

        if email.len() > 254 {
            return Err(InterfaceError::ValidationError(
                "Email too long".to_string(),
            ));
        }

        if !email.contains('@') {
            return Err(InterfaceError::ValidationError(
                "Email must contain @".to_string(),
            ));
        }

        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(InterfaceError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }

        if !parts[1].contains('.') {
            return Err(InterfaceError::ValidationError(
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

impl Deref for InterfaceEmail {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfaceEmail {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InterfaceEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for InterfaceEmail {
    type Err = InterfaceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl TryFrom<Email> for InterfaceEmail {
    type Error = InterfaceError;

    fn try_from(value: Email) -> InterfaceResult<Self> {
        Self::new(&value.email())
    }
}

impl TryFrom<InterfaceEmail> for Email {
    type Error = InterfaceError;

    fn try_from(value: InterfaceEmail) -> InterfaceResult<Self> {
        Self::new(&value.email()).map_err(|err| InterfaceError::Domain(err))
    }
}
