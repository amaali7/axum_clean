use std::collections::HashSet;

use application::dto::user_dto::output::{
    OwnerUserOutput, OwnerUserPreferencesOutput, OwnerUserProfileOutput,
};

use crate::{
    common_objects::{
        role::{permissions::InterfacePermission, InterfaceRoleId},
        user::{status::InterfaceUserStatus, InterfaceUserId},
    },
    error::{InterfaceError, InterfaceResult},
    value_objects::{
        InterfaceBio, InterfaceDateTime, InterfaceEmail, InterfaceLanguage, InterfaceName,
        InterfaceUrl, InterfaceUsername,
    },
    InterfaceAddressess, InterfacePhoneNumbers,
};

/// Owner User Response Data

pub struct OwnerUserResponse {
    pub id: InterfaceUserId,
    pub email: InterfaceEmail,
    pub username: InterfaceUsername,
    pub profile: OwnerUserProfileResponse,
    pub roles: HashSet<InterfaceRoleId>,
    pub permissions: HashSet<InterfacePermission>, // Cached permissions for performance
    pub preferences: OwnerUserPreferencesResponse,
    pub status: InterfaceUserStatus,
}

pub struct OwnerUserPreferencesResponse {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub two_factor_auth: bool,
    pub language: InterfaceLanguage,
}

pub struct OwnerUserProfileResponse {
    pub first_name: InterfaceName,
    pub last_name: InterfaceName,
    pub bio: Option<InterfaceBio>,
    pub phone_numbers: InterfacePhoneNumbers,
    pub avatar_url: Option<InterfaceUrl>,
    pub date_of_birth: Option<InterfaceDateTime>,
    pub addresses: InterfaceAddressess,
    pub website: Option<InterfaceUrl>,
}

impl TryFrom<OwnerUserOutput> for OwnerUserResponse {
    type Error = InterfaceError;

    fn try_from(value: OwnerUserOutput) -> InterfaceResult<Self> {
        Ok(Self {
            id: value.id.into(),
            email: value.email.try_into()?,
            username: value.username.try_into()?,
            profile: OwnerUserProfileResponse::from(value.profile.try_into()?),
            roles: value.roles.into_iter().map(|role| role.into()).collect(),
            permissions: value
                .permissions
                .into_iter()
                .map(|permission| permission.into())
                .collect(),
            preferences: OwnerUserPreferencesResponse::from(value.preferences.try_into()?),
            status: value.status.into(),
        })
    }
}

impl TryFrom<OwnerUserProfileOutput> for OwnerUserProfileResponse {
    type Error = InterfaceError;

    fn try_from(value: OwnerUserProfileOutput) -> InterfaceResult<Self> {
        Ok(Self {
            first_name: value.first_name.try_into()?,
            last_name: value.last_name.try_into()?,
            bio: value.bio.map(|x| x.try_into()).transpose()?,
            phone_numbers: value.phone_numbers.try_into()?,
            avatar_url: value.avatar_url.map(|x| x.try_into()).transpose()?,
            date_of_birth: value.date_of_birth.map(|x| x.try_into()).transpose()?,
            addresses: value.addresses.try_into()?,
            website: value.website.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl TryFrom<OwnerUserPreferencesOutput> for OwnerUserPreferencesResponse {
    type Error = InterfaceError;

    fn try_from(value: OwnerUserPreferencesOutput) -> InterfaceResult<Self> {
        Ok(Self {
            email_notifications: value.email_notifications,
            push_notifications: value.push_notifications,
            two_factor_auth: value.two_factor_auth,
            language: value.language.try_into()?,
        })
    }
}
