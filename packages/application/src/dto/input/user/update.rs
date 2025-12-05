use std::collections::HashSet;

use domain::{
    user::{UserPreferences, UserStatus},
    value_objects::{Addressess, Bio, DateTime, Language, PhoneNumbers, Url},
    Email, Name, Password, Permission, RoleId, User, UserId, UserProfile, Username,
};

use crate::error::ApplicationError;

/// Owner User Output Data

#[derive(Default)]
pub struct UpdateUserInput {
    pub id: UserId,
    pub email: Option<Email>,
    pub username: Option<Username>,
    pub profile: Option<UpdateUserProfileInput>,
    pub roles: HashSet<RoleId>,
    pub permissions: HashSet<Permission>, // Cached permissions for performance
    pub preferences: Option<UpdateUserPreferencesInput>,
    pub status: Option<UserStatus>,
}

#[derive(Default)]
pub struct UpdateUserPreferencesInput {
    pub email_notifications: Option<bool>,
    pub push_notifications: Option<bool>,
    pub two_factor_auth: Option<bool>,
    pub language: Option<Language>,
}

#[derive(Default)]
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
    pub created_at: Option<DateTime>,
    pub updated_at: DateTime,
}

impl From<User> for UpdateUserInput {
    fn from(value: User) -> Self {
        UpdateUserInput {
            email: Some(value.email()),
            username: Some(value.username()),
            profile: Some(UpdateUserProfileInput::from(value.profile())),
            roles: value.roles(),
            permissions: value.permissions(),
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

/// Mapper from DTO

impl TryFrom<UpdateUserInput> for User {
    type Error = ApplicationError;

    fn try_from(value: UpdateUserInput) -> Result<Self, Self::Error> {
        let mut builder = User::new(value.id);
        builder
            .set_email(value.email.unwrap_or_default())
            .set_profile(UserProfile::try_from(value.profile.unwrap_or_default())?)
            .set_username(value.username.unwrap_or_default())
            .set_preferences(UserPreferences::from(value.preferences.unwrap_or_default()))
            .add_permissions(value.permissions)
            .add_roles(value.roles)
            .set_status(value.status.unwrap_or_default());
        let user = builder.build()?;

        Ok(user)
    }
}

impl TryFrom<UpdateUserProfileInput> for UserProfile {
    type Error = ApplicationError;

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
        let user_profile = builder.build(value.created_at.unwrap_or_default(), value.updated_at)?;
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
