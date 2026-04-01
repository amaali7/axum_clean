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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserProfileField {
    FirstName,
    LastName,
    Password,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserPreferencesField {
    EmailNotifications,
    PushNotifications,
    TwoFactorAuth,
    Language,
}
