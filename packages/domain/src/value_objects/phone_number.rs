use std::ops::{Deref, DerefMut};

use crate::{error::DomainResult, DomainError};

#[derive(Debug, Clone, Default)]
pub struct PhoneNumbers(Vec<PhoneNumber>);

impl PhoneNumbers {
    pub fn new(phone_numbers: &[PhoneNumber]) -> Self {
        PhoneNumbers(phone_numbers.to_vec())
    }

    pub fn add_address(&mut self, phone_number: PhoneNumber) {
        self.0.push(phone_number);
    }
}

impl Deref for PhoneNumbers {
    type Target = Vec<PhoneNumber>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PhoneNumbers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// More value objects...
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneNumber {
    title: String,
    number: String,
}

impl PhoneNumber {
    pub fn new(title: &str, number: &str) -> DomainResult<Self> {
        let cleaned = number
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();

        if cleaned.len() < 10 || cleaned.len() > 15 {
            return Err(DomainError::ValidationError(
                "Invalid phone number length".to_string(),
            ));
        }

        if title.len() > 15 {
            return Err(DomainError::ValidationError(
                "Invalid title for phone number length".to_string(),
            ));
        }

        Ok(Self {
            title: title.to_string(),
            number: cleaned,
        })
    }
}

impl std::fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title, self.number)
    }
}
