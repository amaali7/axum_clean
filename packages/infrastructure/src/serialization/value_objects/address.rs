use std::ops::{Deref, DerefMut};

use domain::{Address, Addressess};
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InfrastructureAddressess(Vec<InfrastructureAddress>);

impl InfrastructureAddressess {
    pub fn new() -> Self {
        InfrastructureAddressess(Vec::new())
    }

    pub fn add_addressess(&mut self, address: Self) {
        self.0.extend(address.0);
    }
    pub fn add_address(&mut self, address: InfrastructureAddress) {
        self.0.push(address);
    }

    pub fn addressess(&self) -> Vec<InfrastructureAddress> {
        self.0.clone()
    }
}

impl TryFrom<Addressess> for InfrastructureAddressess {
    fn try_from(value: Addressess) -> InfrastructureResult<Self> {
        let mut addressess = Self::new();
        for address in value.addressess().into_iter() {
            addressess.add_address(InfrastructureAddress::try_from(address)?);
        }
        Ok(addressess)
    }

    type Error = InfrastructureError;
}
impl TryFrom<InfrastructureAddressess> for Addressess {
    fn try_from(value: InfrastructureAddressess) -> InfrastructureResult<Self> {
        let mut addressess = Self::new();
        for address in value.addressess().into_iter() {
            addressess.add_address(Address::try_from(address)?);
        }
        Ok(addressess)
    }

    type Error = InfrastructureError;
}

impl Deref for InfrastructureAddressess {
    type Target = Vec<InfrastructureAddress>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InfrastructureAddressess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureAddress {
    title: String,
    street: String,
    city: String,
    state: String,
    postal_code: String,
    country: String,
}

#[derive(Debug, Default, Clone)]
pub struct InfrastructureAddressBuilder {
    title: Option<String>,
    street: Option<String>,
    city: Option<String>,
    state: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
}

impl TryFrom<Address> for InfrastructureAddress {
    fn try_from(value: Address) -> InfrastructureResult<Self> {
        let mut address_builder = InfrastructureAddress::new();
        address_builder
            .set_title(value.title().as_str())
            .set_city(value.city().as_str())
            .set_country(value.country().as_str())
            .set_postal_code(value.postal_code().as_str())
            .set_state(value.state().as_str())
            .set_street(value.street().as_str());
        address_builder.build()
    }

    type Error = InfrastructureError;
}

impl TryFrom<InfrastructureAddress> for Address {
    fn try_from(value: InfrastructureAddress) -> InfrastructureResult<Self> {
        let mut address_builder = Address::new();
        address_builder
            .set_title(value.title().as_str())
            .set_city(value.city().as_str())
            .set_country(value.country().as_str())
            .set_postal_code(value.postal_code().as_str())
            .set_state(value.state().as_str())
            .set_street(value.street().as_str());
        address_builder
            .build()
            .map_err(|err| InfrastructureError::Domain(err))
    }

    type Error = InfrastructureError;
}

impl InfrastructureAddress {
    pub fn new() -> InfrastructureAddressBuilder {
        InfrastructureAddressBuilder::default()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn street(&self) -> String {
        self.street.clone()
    }
    pub fn city(&self) -> String {
        self.city.clone()
    }
    pub fn state(&self) -> String {
        self.state.clone()
    }
    pub fn postal_code(&self) -> String {
        self.postal_code.clone()
    }

    pub fn country(&self) -> String {
        self.country.clone()
    }
}

impl InfrastructureAddressBuilder {
    pub fn set_title(&mut self, value: &str) -> &mut Self {
        self.title = Some(value.to_string());
        self
    }
    pub fn set_street(&mut self, value: &str) -> &mut Self {
        self.street = Some(value.to_string());
        self
    }
    pub fn set_city(&mut self, value: &str) -> &mut Self {
        self.city = Some(value.to_string());
        self
    }
    pub fn set_state(&mut self, value: &str) -> &mut Self {
        self.state = Some(value.to_string());
        self
    }
    pub fn set_postal_code(&mut self, value: &str) -> &mut Self {
        self.postal_code = Some(value.to_string());
        self
    }
    pub fn set_country(&mut self, value: &str) -> &mut Self {
        self.country = Some(value.to_string());
        self
    }
    pub fn build(self) -> InfrastructureResult<InfrastructureAddress> {
        let title = self.title.clone();
        if title.is_none() {
            return Err(InfrastructureError::ValidationError(
                "Title of Infrastructureaddress is empty".to_string(),
            ));
        } else if title.unwrap().len() > 60 {
            return Err(InfrastructureError::ValidationError(
                "Title of Infrastructureaddress must be < 30 char".to_string(),
            ));
        }
        Ok(InfrastructureAddress {
            title: self.title.unwrap().trim().to_string(),
            street: self.street.unwrap_or("".to_string()),
            city: self.city.unwrap_or("".to_string()),
            state: self.state.unwrap_or("".to_string()),
            postal_code: self.postal_code.unwrap_or("".to_string()),
            country: self.country.unwrap_or("".to_string()),
        })
    }
}
