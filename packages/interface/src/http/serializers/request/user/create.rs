use std::collections::HashSet;

use application::dto::user_dto::input::{
    CreateUserInput, CreateUserPreferencesInput, CreateUserProfileInput,
};
use serde::Deserialize;

use crate::{
    common_objects::{
        role::{permissions::InterfacePermission, InterfaceRoleId},
        user::{status::InterfaceUserStatus, InterfaceUserId},
    },
    error::{InterfaceError, InterfaceResult},
    value_objects::{
        InterfaceAddressess, InterfaceBio, InterfaceDateTime, InterfaceEmail, InterfaceLanguage,
        InterfaceName, InterfacePassword, InterfacePhoneNumbers, InterfaceUrl, InterfaceUsername,
    },
};

/// Owner User Response Data

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserRequest {
    pub id: InterfaceUserId,
    pub email: InterfaceEmail,
    pub username: InterfaceUsername,
    pub profile: CreateUserProfileRequest,
    pub roles: HashSet<InterfaceRoleId>,
    pub permissions: HashSet<InterfacePermission>, // Cached permissions for performance
    pub preferences: CreateUserPreferencesRequest,
    pub status: InterfaceUserStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserPreferencesRequest {
    pub email_notifications: Option<bool>,
    pub push_notifications: Option<bool>,
    pub two_factor_auth: Option<bool>,
    pub language: Option<InterfaceLanguage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserProfileRequest {
    pub first_name: InterfaceName,
    pub last_name: InterfaceName,
    pub bio: Option<InterfaceBio>,
    pub password: InterfacePassword,
    pub phone_numbers: InterfacePhoneNumbers,
    pub avatar_url: Option<InterfaceUrl>,
    pub date_of_birth: Option<InterfaceDateTime>,
    pub addresses: InterfaceAddressess,
    pub website: Option<InterfaceUrl>,
}

// /// Mapper to Application
impl TryFrom<CreateUserRequest> for CreateUserInput {
    type Error = InterfaceError;
    fn try_from(value: CreateUserRequest) -> InterfaceResult<Self> {
        Ok(CreateUserInput {
            email: value.email.try_into()?,
            username: value.username.try_into()?,
            profile: CreateUserProfileInput::try_from(value.profile)?,
            roles: value.roles.into_iter().map(|role| role.into()).collect(),
            permissions: value
                .permissions
                .into_iter()
                .map(|permission| permission.into())
                .collect(),
            preferences: CreateUserPreferencesInput::try_from(value.preferences)?,
            status: value.status.into(),
            id: value.id.into(),
        })
    }
}

impl TryFrom<CreateUserProfileRequest> for CreateUserProfileInput {
    type Error = InterfaceError;
    fn try_from(value: CreateUserProfileRequest) -> InterfaceResult<Self> {
        Ok(CreateUserProfileInput {
            first_name: value.first_name.try_into()?,
            last_name: value.last_name.try_into()?,
            bio: match value.bio {
                Some(bio) => Some(bio.try_into()?),
                None => None,
            },

            phone_numbers: value.phone_numbers.try_into()?,
            avatar_url: match value.avatar_url {
                Some(url) => Some(url.try_into()?),
                None => None,
            },
            date_of_birth: match value.date_of_birth {
                Some(date_of_birth) => Some(date_of_birth.try_into()?),
                None => None,
            },
            addresses: value.addresses.try_into()?,
            website: match value.website {
                Some(website) => Some(website.try_into()?),
                None => None,
            },
            password: value.password.try_into()?,
        })
    }
}

impl TryFrom<CreateUserPreferencesRequest> for CreateUserPreferencesInput {
    type Error = InterfaceError;
    fn try_from(value: CreateUserPreferencesRequest) -> InterfaceResult<Self> {
        Ok(CreateUserPreferencesInput {
            email_notifications: value.email_notifications.unwrap_or_default(),
            push_notifications: value.push_notifications.unwrap_or_default(),
            two_factor_auth: value.two_factor_auth.unwrap_or_default(),
            language: value
                .language
                .map(|x| x.try_into())
                .transpose()?
                .unwrap_or_default(),
        })
    }
}
