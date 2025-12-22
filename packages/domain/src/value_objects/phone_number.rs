use std::ops::{Deref, DerefMut};

use crate::{error::DomainResult, DomainError};

use super::Title;

#[derive(Debug, Clone, Default)]
pub struct PhoneNumbers(Vec<PhoneNumber>);

impl PhoneNumbers {
    pub fn new() -> Self {
        PhoneNumbers(Vec::new())
    }

    pub fn add_phone_numbers(&mut self, phone_numbers: Self) {
        self.0.extend(phone_numbers.0);
    }

    pub fn add_phone_number(&mut self, phone_number: PhoneNumber) {
        self.0.push(phone_number);
    }

    pub fn phone_numbers(&self) -> Vec<PhoneNumber> {
        self.0.clone()
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
    title: Title,
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
            title: Title::new(title)?,
            number: cleaned,
        })
    }
    pub fn title(&self) -> Title {
        self.title.clone()
    }
    pub fn number(&self) -> String {
        self.number.clone()
    }
}

impl std::fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title, self.number)
    }
}
