use std::collections::HashSet;

use application::dto::user_dto::output::{PrivilegeUserOutput, PrivilegeUserProfileOutput};

use crate::{
    common_objects::{
        role::{permissions::InterfacePermission, InterfaceRoleId},
        user::{status::InterfaceUserStatus, InterfaceUserId},
    },
    error::{InterfaceError, InterfaceResult},
    value_objects::{InterfaceBio, InterfaceEmail, InterfaceName, InterfaceUrl, InterfaceUsername},
    InterfaceAddressess, InterfacePhoneNumbers,
};

/// Not owner but have privileges

pub struct PrivilegeUserResponse {
    pub id: InterfaceUserId,
    pub email: InterfaceEmail,
    pub username: InterfaceUsername,
    pub profile: PrivilegeUserProfileResponse,
    pub roles: HashSet<InterfaceRoleId>,
    pub permissions: HashSet<InterfacePermission>, // Cached permissions for performance
    pub status: InterfaceUserStatus,
}

pub struct PrivilegeUserProfileResponse {
    pub first_name: InterfaceName,
    pub last_name: InterfaceName,
    pub bio: Option<InterfaceBio>,
    pub phone_numbers: InterfacePhoneNumbers,
    pub avatar_url: Option<InterfaceUrl>,
    pub addresses: InterfaceAddressess,
    pub website: Option<InterfaceUrl>,
}

impl TryFrom<PrivilegeUserOutput> for PrivilegeUserResponse {
    type Error = InterfaceError;

    fn try_from(value: PrivilegeUserOutput) -> InterfaceResult<Self> {
        Ok(Self {
            id: value.id.into(),
            email: value.email.try_into()?,
            username: value.username.try_into()?,
            profile: value.profile.try_into()?,
            roles: value.roles.into_iter().map(|x| x.into()).collect(),
            permissions: value.permissions.into_iter().map(|x| x.into()).collect(),
            status: value.status.into(),
        })
    }
}

impl TryFrom<PrivilegeUserProfileOutput> for PrivilegeUserProfileResponse {
    type Error = InterfaceError;

    fn try_from(value: PrivilegeUserProfileOutput) -> InterfaceResult<Self> {
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
