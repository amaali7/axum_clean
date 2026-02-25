use domain::{Addressess, DateTime, Password, PhoneNumbers, UserProfile};
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::{
            date_time::InfrastructureDateTime, InfrastructureBio, InfrastructureName, InfrastructureUrl,
        },
        InfrastructureAddressess, InfrastructurePassword, InfrastructurePhoneNumbers,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureUserProfile {
    first_name: InfrastructureName,
    last_name: InfrastructureName,
    password: InfrastructurePassword,
    bio: Option<InfrastructureBio>,
    phone_numbers: InfrastructurePhoneNumbers,
    avatar_url: Option<InfrastructureUrl>,
    date_of_birth: Option<InfrastructureDateTime>,
    addresses: InfrastructureAddressess,
    website: Option<InfrastructureUrl>,
    is_deleted: bool,
    created_at: InfrastructureDateTime,
    updated_at: InfrastructureDateTime,
}
impl InfrastructureUserProfile {
    pub fn first_name(&self) -> InfrastructureName {
        self.first_name.clone()
    }
    pub fn last_name(&self) -> InfrastructureName {
        self.last_name.clone()
    }
    pub fn password(&self) -> InfrastructurePassword {
        self.password.clone()
    }
    pub fn bio(&self) -> Option<InfrastructureBio> {
        self.bio.clone()
    }
    pub fn phone_numbers(&self) -> InfrastructurePhoneNumbers {
        self.phone_numbers.clone()
    }
    pub fn avatar_url(&self) -> Option<InfrastructureUrl> {
        self.avatar_url.clone()
    }
    pub fn date_of_birth(&self) -> Option<InfrastructureDateTime> {
        self.date_of_birth.clone()
    }
    pub fn addresses(&self) -> InfrastructureAddressess {
        self.addresses.clone()
    }
    pub fn website(&self) -> Option<InfrastructureUrl> {
        self.website.clone()
    }
    pub fn is_deleted(&self) -> bool {
        self.is_deleted.clone()
    }
    pub fn created_at(&self) -> InfrastructureDateTime {
        self.created_at.clone()
    }

    pub fn updated_at(&self) -> InfrastructureDateTime {
        self.updated_at.clone()
    }
}

impl TryFrom<UserProfile> for InfrastructureUserProfile {
    fn try_from(profile: UserProfile) -> InfrastructureResult<Self> {
        Ok(Self {
            first_name: profile.first_name().try_into()?,
            last_name: profile.last_name().try_into()?,
            password: InfrastructurePassword::try_from(profile.password())?,
            bio: profile.bio().map_or(None, |value| value.try_into().ok()),
            phone_numbers: InfrastructurePhoneNumbers::try_from(profile.phone_numbers())?,
            avatar_url: profile
                .avatar_url()
                .map_or(None, |value| value.try_into().ok()),
            date_of_birth: {
                match profile.date_of_birth() {
                    Some(datetime) => Some(InfrastructureDateTime::new(datetime)?),
                    None => None,
                }
            },
            addresses: InfrastructureAddressess::try_from(profile.addresses())?,
            website: profile
                .website()
                .map_or(None, |value| value.try_into().ok()),
            is_deleted: profile.is_deleted(),
            created_at: InfrastructureDateTime::new(profile.created_at())?,
            updated_at: InfrastructureDateTime::new(profile.updated_at())?,
        })
    }

    type Error = InfrastructureError;
}

impl TryFrom<InfrastructureUserProfile> for UserProfile {
    fn try_from(profile: InfrastructureUserProfile) -> InfrastructureResult<Self> {
        let is_deleted = profile.is_deleted();
        let created_at = DateTime::try_from(profile.created_at())?;
        let updated_at = DateTime::try_from(profile.updated_at())?;
        let addressess = Addressess::try_from(profile.clone().addresses())?;
        let mut user_brofile_builder = UserProfile::new();
        match profile.bio() {
            Some(bio) => {
                user_brofile_builder.set_bio(bio.try_into()?);
                ()
            }
            None => (),
        };
        match profile.avatar_url() {
            Some(avatar_url) => {
                user_brofile_builder.set_avatar_url(avatar_url.try_into()?);
                ()
            }
            None => (),
        };
        match profile.website() {
            Some(website) => {
                user_brofile_builder.set_website(website.try_into()?);
                ()
            }
            None => (),
        };

        match profile.date_of_birth() {
            Some(date_of_birth) => {
                user_brofile_builder.set_date_of_birth(DateTime::try_from(date_of_birth)?);
                ()
            }
            None => (),
        };
        user_brofile_builder
            .set_first_name(profile.first_name().try_into()?)
            .set_password(Password::try_from(profile.password.clone())?)
            .add_phone_numbers(PhoneNumbers::try_from(profile.phone_numbers())?)
            .set_is_deleted(is_deleted)
            .add_addresss(addressess)
            .set_last_name(profile.last_name().try_into()?);
        Ok(user_brofile_builder.build(created_at, updated_at)?)
    }

    type Error = InfrastructureError;
}
