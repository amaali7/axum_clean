use std::ops::{Deref, DerefMut};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(email: &str) -> DomainResult<Self> {
        let email = email.trim().to_lowercase();

        if email.len() > 254 {
            return Err(DomainError::ValidationError("Email too long".to_string()));
        }

        if !email.contains('@') {
            return Err(DomainError::ValidationError(
                "Email must contain @".to_string(),
            ));
        }

        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(DomainError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }

        if !parts[1].contains('.') {
            return Err(DomainError::ValidationError(
                "Invalid domain in email".to_string(),
            ));
        }

        Ok(Self(email))
    }

    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }

    pub fn is_disposable(&self) -> bool {
        let disposable_domains = ["tempmail.com", "throwaway.com", "guerrillamail.com"];
        disposable_domains
            .iter()
            .any(|domain| self.domain() == *domain)
    }
}

impl Deref for Email {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Email {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
