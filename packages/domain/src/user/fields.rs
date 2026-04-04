use crate::traits::field::Field;

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

impl Field for UserField {
    fn name(&self) -> &'static str {
        match self {
            UserField::Id => todo!(),
            UserField::Email => todo!(),
            UserField::Username => todo!(),
            UserField::Profile(user_profile_field) => todo!(),
            UserField::Preferences(user_preferences_field) => todo!(),
            UserField::Status => todo!(),
            UserField::FailedLogins => todo!(),
            UserField::LockedUntil => todo!(),
            UserField::LastLogin => todo!(),
            UserField::Version => todo!(),
        }
    }
}

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

impl Field for UserProfileField {
    fn name(&self) -> &'static str {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserPreferencesField {
    EmailNotifications,
    PushNotifications,
    TwoFactorAuth,
    Language,
}

impl Field for UserPreferencesField {
    fn name(&self) -> &'static str {
        todo!()
    }
}
