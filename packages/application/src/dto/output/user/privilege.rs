use std::collections::HashSet;

use domain::{
    user::UserStatus,
    value_objects::{Addressess, Bio, PhoneNumbers, Url},
    Email, Name, User, UserId, UserProfile, Username,
};

/// Not owner but have privileges

pub struct PrivilegeUserOutput {
    pub id: UserId,
    pub email: Email,
    pub username: Username,
    pub profile: PrivilegeUserProfileOutput,
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
            id: value.id().clone(),
            email: value.email().clone(),
            username: value.username().clone(),
            profile: PrivilegeUserProfileOutput::from(value.profile().clone()),
            status: value.status().clone(),
        }
    }
}

impl From<UserProfile> for PrivilegeUserProfileOutput {
    fn from(value: UserProfile) -> Self {
        Self {
            first_name: value.first_name().clone(),
            last_name: value.last_name().clone(),
            bio: value.bio().clone(),
            phone_numbers: value.phone_numbers().clone(),
            avatar_url: value.avatar_url().clone(),
            addresses: value.addresses().clone(),
            website: value.website().clone(),
        }
    }
}
