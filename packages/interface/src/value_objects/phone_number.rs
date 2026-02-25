use std::ops::{Deref, DerefMut};

use domain::{PhoneNumber, PhoneNumbers};
use serde::{Deserialize, Serialize};

use crate::error::{InterfaceError, InterfaceResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterfacePhoneNumbers(Vec<InterfacePhoneNumber>);

impl InterfacePhoneNumbers {
    pub fn new() -> Self {
        InterfacePhoneNumbers(Vec::new())
    }

    pub fn add_phone_numbers(&mut self, phone_numbers: Self) {
        self.0.extend(phone_numbers.0);
    }

    pub fn add_phone_number(&mut self, phone_number: InterfacePhoneNumber) {
        self.0.push(phone_number);
    }
    pub fn phone_numbers(&self) -> Vec<InterfacePhoneNumber> {
        self.0.clone()
    }
}

impl TryFrom<PhoneNumbers> for InterfacePhoneNumbers {
    fn try_from(value: PhoneNumbers) -> InterfaceResult<Self> {
        let mut phone_numbers_record = Self::new();
        for phone_number in value.phone_numbers().into_iter() {
            phone_numbers_record.add_phone_number(InterfacePhoneNumber::try_from(phone_number)?);
        }
        Ok(phone_numbers_record)
    }

    type Error = InterfaceError;
}

impl TryFrom<InterfacePhoneNumbers> for PhoneNumbers {
    fn try_from(value: InterfacePhoneNumbers) -> InterfaceResult<Self> {
        let mut phone_numbers_record = Self::new();
        for phone_number in value.phone_numbers().into_iter() {
            phone_numbers_record.add_phone_number(PhoneNumber::try_from(phone_number)?);
        }
        Ok(phone_numbers_record)
    }

    type Error = InterfaceError;
}

impl Deref for InterfacePhoneNumbers {
    type Target = Vec<InterfacePhoneNumber>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfacePhoneNumbers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// More value objects...
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterfacePhoneNumber {
    title: String,
    number: String,
}

impl InterfacePhoneNumber {
    pub fn new(title: &str, number: &str) -> InterfaceResult<Self> {
        let cleaned = number
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();

        if cleaned.len() < 10 || cleaned.len() > 15 {
            return Err(InterfaceError::ValidationError(
                "Invalid phone number length".to_string(),
            ));
        }

        if title.len() > 15 {
            return Err(InterfaceError::ValidationError(
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

impl TryFrom<PhoneNumber> for InterfacePhoneNumber {
    fn try_from(value: PhoneNumber) -> InterfaceResult<Self> {
        Ok(InterfacePhoneNumber::new(
            &value.title().to_string(),
            value.number().as_str(),
        )?)
    }

    type Error = InterfaceError;
}

impl TryFrom<InterfacePhoneNumber> for PhoneNumber {
    fn try_from(value: InterfacePhoneNumber) -> InterfaceResult<Self> {
        Ok(PhoneNumber::new(
            &value.title().to_string(),
            value.number().as_str(),
        )?)
    }

    type Error = InterfaceError;
}

impl std::fmt::Display for InterfacePhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title, self.number)
    }
}
