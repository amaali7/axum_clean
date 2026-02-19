use std::collections::HashSet;

use domain::{
    events::DomainEventId,
    user::UserStatus,
    value_objects::{Addressess, Bio, PhoneNumbers, Url},
    Email, Name, Permission, RoleId, User, UserId, UserProfile, Username,
};

/// Not owner but have privileges

pub struct PrivilegeUserOutput {
    pub id: UserId,
    pub email: Email,
    pub username: Username,
    pub profile: PrivilegeUserProfileOutput,
    pub roles: HashSet<RoleId>,
    pub permissions: HashSet<Permission>, // Cached permissions for performance
    pub status: UserStatus,
}

pub struct PrivilegeUserProfileOutput {
    pub first_name: Name,
    pub last_name: Name,
    pub bio: Option<Bio>,
    pub phone_numbers: PhoneNumbers,
    pub avatar_url: Option<Url>,
    pub addresses: Addressess,
    pub website: Option<Url>,
}

impl From<User> for PrivilegeUserOutput {
    fn from(value: User) -> Self {
        Self {
            id: value.id(),
            email: value.email(),
            username: value.username(),
            profile: PrivilegeUserProfileOutput::from(value.profile()),
            roles: value.roles(),
            permissions: value.permissions(),
            status: value.status(),
        }
    }
}

impl From<UserProfile> for PrivilegeUserProfileOutput {
    fn from(value: UserProfile) -> Self {
        Self {
            first_name: value.first_name(),
            last_name: value.last_name(),
            bio: value.bio(),
            phone_numbers: value.phone_numbers(),
            avatar_url: value.avatar_url(),
            addresses: value.addresses(),
            website: value.website(),
        }
    }
}
