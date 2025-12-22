use std::ops::{Deref, DerefMut};

use domain::{PhoneNumber, PhoneNumbers};
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SerializedPhoneNumbers(Vec<SerializedPhoneNumber>);

impl SerializedPhoneNumbers {
    pub fn new() -> Self {
        SerializedPhoneNumbers(Vec::new())
    }

    pub fn add_phone_numbers(&mut self, phone_numbers: Self) {
        self.0.extend(phone_numbers.0);
    }

    pub fn add_phone_number(&mut self, phone_number: SerializedPhoneNumber) {
        self.0.push(phone_number);
    }
    pub fn phone_numbers(&self) -> Vec<SerializedPhoneNumber> {
        self.0.clone()
    }
}

impl TryFrom<PhoneNumbers> for SerializedPhoneNumbers {
    fn try_from(value: PhoneNumbers) -> InfrastructureResult<Self> {
        let mut phone_numbers_record = Self::new();
        for phone_number in value.phone_numbers().into_iter() {
            phone_numbers_record.add_phone_number(SerializedPhoneNumber::try_from(phone_number)?);
        }
        Ok(phone_numbers_record)
    }

    type Error = InfrastructureError;
}

impl TryFrom<SerializedPhoneNumbers> for PhoneNumbers {
    fn try_from(value: SerializedPhoneNumbers) -> InfrastructureResult<Self> {
        let mut phone_numbers_record = Self::new();
        for phone_number in value.phone_numbers().into_iter() {
            phone_numbers_record.add_phone_number(PhoneNumber::try_from(phone_number)?);
        }
        Ok(phone_numbers_record)
    }

    type Error = InfrastructureError;
}

impl Deref for SerializedPhoneNumbers {
    type Target = Vec<SerializedPhoneNumber>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializedPhoneNumbers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// More value objects...
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SerializedPhoneNumber {
    title: String,
    number: String,
}

impl SerializedPhoneNumber {
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

impl TryFrom<PhoneNumber> for SerializedPhoneNumber {
    fn try_from(value: PhoneNumber) -> InfrastructureResult<Self> {
        Ok(SerializedPhoneNumber::new(
            &value.title().to_string(),
            value.number().as_str(),
        )?)
    }

    type Error = InfrastructureError;
}

impl TryFrom<SerializedPhoneNumber> for PhoneNumber {
    fn try_from(value: SerializedPhoneNumber) -> InfrastructureResult<Self> {
        Ok(PhoneNumber::new(
            &value.title().to_string(),
            value.number().as_str(),
        )?)
    }

    type Error = InfrastructureError;
}

impl std::fmt::Display for SerializedPhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title, self.number)
    }
}
