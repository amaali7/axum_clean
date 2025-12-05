use std::collections::HashSet;

use domain::{
    events::DomainEventId,
    user::{UserPreferences, UserStatus},
    value_objects::{Addressess, Bio, DateTime, Language, PhoneNumbers, Url},
    Email, Name, Permission, RoleId, User, UserId, UserProfile, Username,
};

/// Owner User Output Data

pub struct OwnerUserOutput {
    pub id: UserId,
    pub email: Email,
    pub username: Username,
    pub profile: OwnerUserProfileOutput,
    pub roles: HashSet<RoleId>,
    pub permissions: HashSet<Permission>, // Cached permissions for performance
    pub preferences: OwnerUserPreferencesOutput,
    pub status: UserStatus,
    pub events: Vec<DomainEventId>,
}

pub struct OwnerUserPreferencesOutput {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub two_factor_auth: bool,
    pub language: Language,
}

pub struct OwnerUserProfileOutput {
    pub first_name: Name,
    pub last_name: Name,
    pub bio: Option<Bio>,
    pub phone_numbers: PhoneNumbers,
    pub avatar_url: Option<Url>,
    pub date_of_birth: Option<DateTime>,
    pub addresses: Addressess,
    pub website: Option<Url>,
}

impl From<User> for OwnerUserOutput {
    fn from(value: User) -> Self {
        Self {
            id: value.id().clone(),
            email: value.email().clone(),
            username: value.username().clone(),
            profile: OwnerUserProfileOutput::from(value.profile().clone()),
            roles: value.roles().clone(),
            permissions: value.permissions().clone(),
            preferences: OwnerUserPreferencesOutput::from(value.preferences().clone()),
            status: value.status().clone(),
            events: value.events(),
        }
    }
}

impl From<UserProfile> for OwnerUserProfileOutput {
    fn from(value: UserProfile) -> Self {
        Self {
            first_name: value.first_name().clone(),
            last_name: value.last_name().clone(),
            bio: value.bio().clone(),
            phone_numbers: value.phone_numbers().clone(),
            avatar_url: value.avatar_url().clone(),
            date_of_birth: value.date_of_birth().clone(),
            addresses: value.addresses().clone(),
            website: value.website().clone(),
        }
    }
}

impl From<UserPreferences> for OwnerUserPreferencesOutput {
    fn from(value: UserPreferences) -> Self {
        Self {
            email_notifications: value.email_notifications().clone(),
            push_notifications: value.push_notifications().clone(),
            two_factor_auth: value.two_factor_auth().clone(),
            language: value.language().clone(),
        }
    }
}
