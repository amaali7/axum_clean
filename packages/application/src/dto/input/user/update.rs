use domain::{
    user::{UserPreferences, UserStatus},
    value_objects::{Addressess, Bio, DateTime, Language, PhoneNumbers, Url},
    Email, Name, Password, User, UserId, UserProfile, Username,
};

use crate::error::AppError;

/// Owner User Output Data

#[derive(Default, Clone)]
pub struct UpdateUserInput {
    pub id: UserId,
    pub email: Option<Email>,
    pub username: Option<Username>,
    pub profile: Option<UpdateUserProfileInput>,
    pub preferences: Option<UpdateUserPreferencesInput>,
    pub status: Option<UserStatus>,
}

#[derive(Default, Clone)]
pub struct UpdateUserPreferencesInput {
    pub email_notifications: Option<bool>,
    pub push_notifications: Option<bool>,
    pub two_factor_auth: Option<bool>,
    pub language: Option<Language>,
}

#[derive(Default, Clone)]
pub struct UpdateUserProfileInput {
    pub first_name: Option<Name>,
    pub last_name: Option<Name>,
    pub bio: Option<Bio>,
    pub password: Option<Password>,
    pub phone_numbers: PhoneNumbers,
    pub avatar_url: Option<Url>,
    pub date_of_birth: Option<DateTime>,
    pub addresses: Addressess,
    pub website: Option<Url>,
}

/// Mapper from DTO

impl TryFrom<UpdateUserInput> for User {
    type Error = AppError;

    fn try_from(value: UpdateUserInput) -> Result<Self, Self::Error> {
        let mut builder = User::new(value.id);
        builder
            .set_email(value.email.unwrap_or_default().clone())
            .set_profile(UserProfile::try_from(
                value.profile.unwrap_or_default().clone(),
            )?)
            .set_username(value.username.unwrap_or_default().clone())
            .set_preferences(UserPreferences::from(
                value.preferences.unwrap_or_default().clone(),
            ))
            .set_status(value.status.unwrap_or_default().clone());
        let user = builder.build()?;

        Ok(user)
    }
}

impl TryFrom<UpdateUserProfileInput> for UserProfile {
    type Error = AppError;

    fn try_from(value: UpdateUserProfileInput) -> Result<Self, Self::Error> {
        let mut builder = UserProfile::new();
        builder
            .set_avatar_url(value.avatar_url.unwrap_or_default())
            .set_bio(value.bio.unwrap_or_default())
            .add_addresss(value.addresses)
            .add_phone_numbers(value.phone_numbers)
            .set_date_of_birth(value.date_of_birth.unwrap_or_default())
            .set_first_name(value.first_name.unwrap_or_default())
            .set_last_name(value.last_name.unwrap_or_default())
            .set_password(value.password.unwrap_or_default());
        let user_profile = builder.build(DateTime::new(0), DateTime::new(0))?;
        Ok(user_profile)
    }
}

impl From<UpdateUserPreferencesInput> for UserPreferences {
    fn from(value: UpdateUserPreferencesInput) -> Self {
        Self::new(
            value.email_notifications.unwrap_or_default(),
            value.push_notifications.unwrap_or_default(),
            value.two_factor_auth.unwrap_or_default(),
            value.language.unwrap_or_default(),
        )
    }
}
