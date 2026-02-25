use std::ops::{Deref, DerefMut};

use domain::{PhoneNumber, PhoneNumbers};
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InfrastructurePhoneNumbers(Vec<InfrastructurePhoneNumber>);

impl InfrastructurePhoneNumbers {
    pub fn new() -> Self {
        InfrastructurePhoneNumbers(Vec::new())
    }

    pub fn add_phone_numbers(&mut self, phone_numbers: Self) {
        self.0.extend(phone_numbers.0);
    }

    pub fn add_phone_number(&mut self, phone_number: InfrastructurePhoneNumber) {
        self.0.push(phone_number);
    }
    pub fn phone_numbers(&self) -> Vec<InfrastructurePhoneNumber> {
        self.0.clone()
    }
}

impl TryFrom<PhoneNumbers> for InfrastructurePhoneNumbers {
    fn try_from(value: PhoneNumbers) -> InfrastructureResult<Self> {
        let mut phone_numbers_record = Self::new();
        for phone_number in value.phone_numbers().into_iter() {
            phone_numbers_record.add_phone_number(InfrastructurePhoneNumber::try_from(phone_number)?);
        }
        Ok(phone_numbers_record)
    }

    type Error = InfrastructureError;
}

impl TryFrom<InfrastructurePhoneNumbers> for PhoneNumbers {
    fn try_from(value: InfrastructurePhoneNumbers) -> InfrastructureResult<Self> {
        let mut phone_numbers_record = Self::new();
        for phone_number in value.phone_numbers().into_iter() {
            phone_numbers_record.add_phone_number(PhoneNumber::try_from(phone_number)?);
        }
        Ok(phone_numbers_record)
    }

    type Error = InfrastructureError;
}

impl Deref for InfrastructurePhoneNumbers {
    type Target = Vec<InfrastructurePhoneNumber>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InfrastructurePhoneNumbers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// More value objects...
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InfrastructurePhoneNumber {
    title: String,
    number: String,
}

impl InfrastructurePhoneNumber {
    pub fn new(title: &str, number: &str) -> InfrastructureResult<Self> {
        let cleaned = number
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();

        if cleaned.len() < 10 || cleaned.len() > 15 {
            return Err(InfrastructureError::ValidationError(
                "Invalid phone number length".to_string(),
            ));
        }

        if title.len() > 15 {
            return Err(InfrastructureError::ValidationError(
                "Invalid title for phone number length".to_string(),
            ));
        }

        Ok(Self {
            title: title.to_string(),
            number: cleaned,
        })
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn number(&self) -> String {
        self.number.clone()
    }
}

impl TryFrom<PhoneNumber> for InfrastructurePhoneNumber {
    fn try_from(value: PhoneNumber) -> InfrastructureResult<Self> {
        Ok(InfrastructurePhoneNumber::new(
            &value.title().to_string(),
            value.number().as_str(),
        )?)
    }

    type Error = InfrastructureError;
}

impl TryFrom<InfrastructurePhoneNumber> for PhoneNumber {
    fn try_from(value: InfrastructurePhoneNumber) -> InfrastructureResult<Self> {
        Ok(PhoneNumber::new(
            &value.title().to_string(),
            value.number().as_str(),
        )?)
    }

    type Error = InfrastructureError;
}

impl std::fmt::Display for InfrastructurePhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title, self.number)
    }
}
