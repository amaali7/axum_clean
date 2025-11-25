use chrono::{DateTime, NaiveDate, Utc};

use crate::{
    value_objects::{address::Addressess, phone_number::PhoneNumbers, Bio, Url},
    DomainError, Name, Password,
};

#[derive(Debug, Clone)]
pub struct UserProfile {
    first_name: Name,
    last_name: Name,
    password: Password,
    bio: Option<Bio>,
    phone_numbers: Option<PhoneNumbers>,
    avatar_url: Option<Url>,
    date_of_birth: Option<NaiveDate>,
    addresses: Option<Addressess>,
    website: Option<Url>,
    is_deleted: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UserProfileBuilder {
    first_name: Option<Name>,
    last_name: Option<Name>,
    password: Option<Password>,
    bio: Option<Bio>,
    avatar_url: Option<Url>,
    date_of_birth: Option<NaiveDate>,
    addressess: Option<Addressess>,
    website: Option<Url>,
    phone_numbers: Option<PhoneNumbers>,
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
            addressess: None,
            website: None,
            phone_numbers: None,
        }
    }

    pub fn set_first_name(mut self, name: Name) -> Self {
        self.first_name = Some(name);
        self
    }

    pub fn set_last_name(mut self, name: Name) -> Self {
        self.first_name = Some(name);
        self
    }

    pub fn set_password(mut self, password: Password) -> Self {
        self.password = Some(password);
        self
    }

    pub fn set_bio(mut self, bio: Bio) -> Self {
        self.bio = Some(bio);
        self
    }

    pub fn set_avatar_url(mut self, url: Url) -> Self {
        self.avatar_url = Some(url);
        self
    }

    pub fn set_date_of_birth(mut self, date: NaiveDate) -> Self {
        self.date_of_birth = Some(date);
        self
    }

    pub fn set_addresss(mut self, addressess: Addressess) -> Self {
        self.addressess = Some(addressess);
        self
    }

    pub fn set_phone_numbers(mut self, phone_numbers: PhoneNumbers) -> Self {
        self.phone_numbers = Some(phone_numbers);
        self
    }

    pub fn build(self) -> Result<UserProfile, DomainError> {
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
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}
