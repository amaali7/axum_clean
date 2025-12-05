use crate::{
    error::DomainResult,
    value_objects::{address::Addressess, phone_number::PhoneNumbers, Bio, DateTime, Url},
    Address, DomainError, Name, Password, PhoneNumber,
};

#[derive(Debug, Clone, Default)]
pub struct UserProfile {
    first_name: Name,
    last_name: Name,
    password: Password,
    bio: Option<Bio>,
    phone_numbers: PhoneNumbers,
    avatar_url: Option<Url>,
    date_of_birth: Option<DateTime>,
    addresses: Addressess,
    website: Option<Url>,
    is_deleted: bool,
    created_at: DateTime,
    updated_at: DateTime,
}

impl UserProfile {
    pub fn new() -> UserProfileBuilder {
        UserProfileBuilder::new()
    }

    pub fn first_name(&self) -> Name {
        self.first_name.clone()
    }
    pub fn last_name(&self) -> Name {
        self.last_name.clone()
    }
    pub fn password(&self) -> Password {
        self.password.clone()
    }
    pub fn bio(&self) -> Option<Bio> {
        self.bio.clone()
    }
    pub fn phone_numbers(&self) -> PhoneNumbers {
        self.phone_numbers.clone()
    }
    pub fn avatar_url(&self) -> Option<Url> {
        self.avatar_url.clone()
    }
    pub fn date_of_birth(&self) -> Option<DateTime> {
        self.date_of_birth.clone()
    }
    pub fn addresses(&self) -> Addressess {
        self.addresses.clone()
    }
    pub fn website(&self) -> Option<Url> {
        self.website.clone()
    }
    pub fn is_deleted(&self) -> bool {
        self.is_deleted.clone()
    }
    pub fn created_at(&self) -> DateTime {
        self.created_at.clone()
    }

    pub fn updated_at(&self) -> DateTime {
        self.updated_at.clone()
    }
}

#[derive(Debug, Clone)]
pub struct UserProfileBuilder {
    first_name: Option<Name>,
    last_name: Option<Name>,
    password: Option<Password>,
    bio: Option<Bio>,
    avatar_url: Option<Url>,
    date_of_birth: Option<DateTime>,
    addressess: Addressess,
    website: Option<Url>,
    phone_numbers: PhoneNumbers,
}

impl UserProfileBuilder {
    pub fn new() -> Self {
        Self {
            first_name: None,
            last_name: None,
            password: None,
            bio: None,
            avatar_url: None,
            date_of_birth: None,
            addressess: Addressess::new(),
            website: None,
            phone_numbers: PhoneNumbers::new(),
        }
    }

    pub fn set_first_name(&mut self, name: Name) -> &mut Self {
        self.first_name = Some(name);
        self
    }

    pub fn set_last_name(&mut self, name: Name) -> &mut Self {
        self.first_name = Some(name);
        self
    }

    pub fn set_password(&mut self, password: Password) -> &mut Self {
        self.password = Some(password);
        self
    }

    pub fn set_bio(&mut self, bio: Bio) -> &mut Self {
        self.bio = Some(bio);
        self
    }

    pub fn set_avatar_url(&mut self, url: Url) -> &mut Self {
        self.avatar_url = Some(url);
        self
    }

    pub fn set_date_of_birth(&mut self, date: DateTime) -> &mut Self {
        self.date_of_birth = Some(date);
        self
    }

    pub fn add_address(&mut self, address: Address) -> &mut Self {
        self.addressess.add_address(address);
        self
    }

    pub fn add_phone_number(&mut self, phone_number: PhoneNumber) -> &mut Self {
        self.phone_numbers.add_phone_number(phone_number);
        self
    }

    pub fn add_addresss(&mut self, addressess: Addressess) -> &mut Self {
        self.addressess.add_addressess(addressess);
        self
    }

    pub fn add_phone_numbers(&mut self, phone_numbers: PhoneNumbers) -> &mut Self {
        self.phone_numbers.add_phone_numbers(phone_numbers);
        self
    }

    pub fn build(self, created_at: DateTime, updated_at: DateTime) -> DomainResult<UserProfile> {
        Ok(UserProfile {
            first_name: self.first_name.unwrap(),
            last_name: self.last_name.unwrap(),
            password: self.password.unwrap(),
            bio: self.bio,
            phone_numbers: self.phone_numbers,
            avatar_url: self.avatar_url,
            date_of_birth: Some(self.date_of_birth.ok_or(DomainError::ValidationError(
                "Date of Birth is not Optional !".to_string(),
            ))?),
            addresses: self.addressess,
            website: self.website,
            is_deleted: false,
            created_at: created_at,
            updated_at: updated_at,
        })
    }
}
