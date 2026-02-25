use application::dto::user_dto::output::{GeneralUserOutput, GeneralUserProfileOutput};

use crate::{
    common_objects::user::InterfaceUserId,
    error::{InterfaceError, InterfaceResult},
    value_objects::{InterfaceBio, InterfaceEmail, InterfaceName, InterfaceUrl, InterfaceUsername},
    InterfaceAddressess, InterfacePhoneNumbers,
};

/// General view for every one

pub struct GeneralUserResponse {
    pub id: InterfaceUserId,
    pub email: InterfaceEmail,
    pub username: InterfaceUsername,
    pub profile: GeneralUserProfileResponse,
}

pub struct GeneralUserProfileResponse {
    pub first_name: InterfaceName,
    pub last_name: InterfaceName,
    pub bio: Option<InterfaceBio>,
    pub phone_numbers: InterfacePhoneNumbers,
    pub avatar_url: Option<InterfaceUrl>,
    pub addresses: InterfaceAddressess,
    pub website: Option<InterfaceUrl>,
}

impl TryFrom<GeneralUserOutput> for GeneralUserResponse {
    type Error = InterfaceError;
    fn try_from(value: GeneralUserOutput) -> InterfaceResult<Self> {
        Ok(Self {
            id: value.id.into(),
            email: value.email.try_into()?,
            username: value.username.try_into()?,
            profile: value.profile.try_into()?,
        })
    }
}

impl TryFrom<GeneralUserProfileOutput> for GeneralUserProfileResponse {
    type Error = InterfaceError;
    fn try_from(value: GeneralUserProfileOutput) -> InterfaceResult<Self> {
        Ok(Self {
            first_name: value.first_name.try_into()?,

            last_name: value.last_name.try_into()?,
            bio: value.bio.map(|x| x.try_into()).transpose()?,
            phone_numbers: value.phone_numbers.try_into()?,
            avatar_url: value.avatar_url.map(|x| x.try_into()).transpose()?,
            addresses: value.addresses.try_into()?,
            website: value.website.map(|x| x.try_into()).transpose()?,
        })
    }
}
