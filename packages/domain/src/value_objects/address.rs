use std::ops::{Deref, DerefMut};

use crate::DomainError;

#[derive(Debug, Clone, Default)]
pub struct Addressess(Vec<Address>);

impl Addressess {
    pub fn new(addressess: &[Address]) -> Self {
        Addressess(addressess.to_vec())
    }

    pub fn add_address(&mut self, address: Address) {
        self.0.push(address);
    }
}

impl Deref for Addressess {
    type Target = Vec<Address>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Addressess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct Address {
    title: String,
    street: String,
    city: String,
    state: String,
    postal_code: String,
    country: String,
}

#[derive(Debug, Default, Clone)]
pub struct AddressBuilder {
    title: Option<String>,
    street: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
}

impl Address {
    pub fn builder() -> AddressBuilder {
        AddressBuilder::default()
    }
}

impl AddressBuilder {
    pub fn set_title(mut self, value: &str) -> Self {
        self.title = Some(value.to_string());
        self
    }
    pub fn set_street(mut self, value: &str) -> Self {
        self.street = Some(value.to_string());
        self
    }
    pub fn set_city(mut self, value: &str) -> Self {
        self.city = Some(value.to_string());
        self
    }
    pub fn set_state(mut self, value: &str) -> Self {
        self.state = Some(value.to_string());
        self
    }
    pub fn set_postal_code(mut self, value: &str) -> Self {
        self.postal_code = Some(value.to_string());
        self
    }
    pub fn set_country(mut self, value: &str) -> Self {
        self.country = Some(value.to_string());
        self
    }
    pub fn build(self) -> Result<Address, DomainError> {
        let title = self.title.clone();
        if title.is_none() {
            return Err(DomainError::ValidationError(
                "Title of address is empty".to_string(),
            ));
        } else if title.unwrap().len() > 60 {
            return Err(DomainError::ValidationError(
                "Title of address must be < 30 char".to_string(),
            ));
        }
        Ok(Address {
            title: self.title.unwrap().trim().to_string(),
            street: self.street.unwrap_or("".to_string()),
            city: self.city.unwrap_or("".to_string()),
            state: self.state.unwrap_or("".to_string()),
            postal_code: self.postal_code.unwrap_or("".to_string()),
            country: self.country.unwrap_or("".to_string()),
        })
    }
}
