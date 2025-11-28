use std::collections::HashSet;

use domain::{
    user::{UserPreferences, UserStatus},
    value_objects::{Addressess, Bio, DateTime, PhoneNumbers, Url},
    Email, Name, Password, Permission, RoleId, User, UserId, UserProfile, Username,
};

/// Owner User Output Data

pub struct UpdateUserInput {
    pub id: UserId,
    pub email: Option<Email>,
    pub username: Option<Username>,
    pub profile: Option<UpdateUserProfileInput>,
    pub roles: Option<HashSet<RoleId>>,
    pub permissions: Option<HashSet<Permission>>, // Cached permissions for performance
    pub preferences: Option<UpdateUserPreferencesInput>,
    pub status: Option<UserStatus>,
}

pub struct UpdateUserPreferencesInput {
    pub email_notifications: Option<bool>,
    pub push_notifications: Option<bool>,
    pub two_factor_auth: Option<bool>,
    pub language: Option<String>,
}

pub struct UpdateUserProfileInput {
    pub first_name: Option<Name>,
    pub last_name: Option<Name>,
    pub bio: Option<Bio>,
    pub password: Option<Password>,
    pub phone_numbers: Option<PhoneNumbers>,
    pub avatar_url: Option<Url>,
    pub date_of_birth: Option<DateTime>,
    pub addresses: Option<Addressess>,
    pub website: Option<Url>,
    pub created_at: Option<DateTime>,
    pub updated_at: DateTime,
}

impl From<User> for UpdateUserInput {
    fn from(value: User) -> Self {
        UpdateUserInput {
            email: Some(value.email()),
            username: Some(value.username()),
            profile: Some(UpdateUserProfileInput::from(value.profile())),
            roles: Some(value.roles()),
            permissions: Some(value.permissions()),
            preferences: Some(UpdateUserPreferencesInput::from(value.preferences())),
            status: Some(value.status()),
            id: value.id(),
        }
    }
}

impl From<UserProfile> for UpdateUserProfileInput {
    fn from(value: UserProfile) -> Self {
        UpdateUserProfileInput {
            first_name: Some(value.first_name()),
            last_name: Some(value.last_name()),
            bio: value.bio(),
            phone_numbers: value.phone_numbers(),
            avatar_url: value.avatar_url(),
            date_of_birth: value.date_of_birth(),
            addresses: value.addresses(),
            website: value.website(),
            password: Some(value.password()),
            created_at: Some(value.created_at()),
            updated_at: value.updated_at(),
        }
    }
}

impl From<UserPreferences> for UpdateUserPreferencesInput {
    fn from(value: UserPreferences) -> Self {
        UpdateUserPreferencesInput {
            email_notifications: Some(value.email_notifications()),
            push_notifications: Some(value.push_notifications()),
            two_factor_auth: Some(value.two_factor_auth()),
            language: Some(value.language()),
        }
    }
}
