use crate::authorization::access_descriptor::AccessableField;

pub mod projector;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserField {
    Id,
    Email,
    Username,
    Profile(UserProfileField),
    Preferences(UserPreferencesField),
    Status,
    FailedLogins,
    LockedUntil,
    LastLogin,
    Version,
}

impl AccessableField for UserField {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserProfileField {
    FirstName,
    LastName,
    Bio,
    PhoneNumbers,
    AvatarUrl,
    DateOfBirth,
    Addressess,
    Website,
    IsDeleted,
    CreatedAt,
    UpdatedAt,
}

impl AccessableField for UserProfileField {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserPreferencesField {
    EmailNotifications,
    PushNotifications,
    TwoFactorAuth,
    Language,
}

impl AccessableField for UserPreferencesField {}
