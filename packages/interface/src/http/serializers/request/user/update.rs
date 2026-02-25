use std::collections::HashSet;

use application::dto::user_dto::input::{
    UpdateUserInput, UpdateUserPreferencesInput, UpdateUserProfileInput,
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

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateUserRequest {
    pub id: InterfaceUserId,
    pub email: Option<InterfaceEmail>,
    pub username: Option<InterfaceUsername>,
    pub profile: Option<UpdateUserProfileRequest>,
    pub roles: HashSet<InterfaceRoleId>,
    pub permissions: HashSet<InterfacePermission>, // Cached permissions for performance
    pub preferences: Option<UpdateUserPreferencesRequest>,
    pub status: Option<InterfaceUserStatus>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateUserPreferencesRequest {
    pub email_notifications: Option<bool>,
    pub push_notifications: Option<bool>,
    pub two_factor_auth: Option<bool>,
    pub language: Option<InterfaceLanguage>,
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct UpdateUserProfileRequest {
    pub first_name: Option<InterfaceName>,
    pub last_name: Option<InterfaceName>,
    pub bio: Option<InterfaceBio>,
    pub password: Option<InterfacePassword>,
    pub phone_numbers: InterfacePhoneNumbers,
    pub avatar_url: Option<InterfaceUrl>,
    pub date_of_birth: Option<InterfaceDateTime>,
    pub addresses: InterfaceAddressess,
    pub website: Option<InterfaceUrl>,
}

impl TryFrom<UpdateUserRequest> for UpdateUserInput {
    type Error = InterfaceError;
    fn try_from(value: UpdateUserRequest) -> InterfaceResult<Self> {
        Ok(UpdateUserInput {
            email: value.email.map(|a| a.try_into()).transpose()?,
            username: value.username.map(|a| a.try_into()).transpose()?,
            profile: value.profile.map(|a| a.try_into()).transpose()?,
            roles: value.roles.into_iter().map(|role| role.into()).collect(),
            permissions: value
                .permissions
                .into_iter()
                .map(|permission| permission.into())
                .collect(),
            preferences: value.preferences.map(|a| a.try_into()).transpose()?,
            status: value.status.map(|a| a.into()),
            id: value.id.into(),
        })
    }
}

impl TryFrom<UpdateUserProfileRequest> for UpdateUserProfileInput {
    type Error = InterfaceError;
    fn try_from(value: UpdateUserProfileRequest) -> InterfaceResult<Self> {
        Ok(UpdateUserProfileInput {
            first_name: value.first_name.map(|a| a.try_into()).transpose()?,
            last_name: value.last_name.map(|a| a.try_into()).transpose()?,
            bio: value.bio.map(|a| a.try_into()).transpose()?,

            phone_numbers: value.phone_numbers.try_into()?,
            avatar_url: value.avatar_url.map(|a| a.try_into()).transpose()?,
            date_of_birth: value.date_of_birth.map(|a| a.try_into()).transpose()?,
            addresses: value.addresses.try_into()?,
            website: value.website.map(|a| a.try_into()).transpose()?,
            password: value.password.map(|a| a.try_into()).transpose()?,
        })
    }
}

impl TryFrom<UpdateUserPreferencesRequest> for UpdateUserPreferencesInput {
    type Error = InterfaceError;
    fn try_from(value: UpdateUserPreferencesRequest) -> InterfaceResult<Self> {
        Ok(UpdateUserPreferencesInput {
            email_notifications: value.email_notifications,
            push_notifications: value.push_notifications,
            two_factor_auth: value.two_factor_auth,
            language: value.language.map(|a| a.try_into()).transpose()?,
        })
    }
}
