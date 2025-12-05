use std::collections::HashSet;

use domain::{
    user::{UserPreferences, UserStatus},
    value_objects::{Addressess, Bio, DateTime, Language, PhoneNumbers, Url},
    Email, Name, Password, Permission, RoleId, User, UserId, UserProfile, Username,
};

use crate::error::ApplicationError;

/// Owner User Output Data

pub struct CreateUserInput {
    pub id: UserId,
    pub email: Email,
    pub username: Username,
    pub profile: CreateUserProfileInput,
    pub roles: HashSet<RoleId>,
    pub permissions: HashSet<Permission>, // Cached permissions for performance
    pub preferences: CreateUserPreferencesInput,
    pub status: UserStatus,
}

pub struct CreateUserPreferencesInput {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub two_factor_auth: bool,
    pub language: Language,
}

pub struct CreateUserProfileInput {
    pub first_name: Name,
    pub last_name: Name,
    pub bio: Option<Bio>,
    pub password: Password,
    pub phone_numbers: PhoneNumbers,
    pub avatar_url: Option<Url>,
    pub date_of_birth: Option<DateTime>,
    pub addresses: Addressess,
    pub website: Option<Url>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

/// Mapper from Domain
impl From<User> for CreateUserInput {
    fn from(value: User) -> Self {
        CreateUserInput {
            email: value.email(),
            username: value.username(),
            profile: CreateUserProfileInput::from(value.profile()),
            roles: value.roles(),
            permissions: value.permissions(),
            preferences: CreateUserPreferencesInput::from(value.preferences()),
            status: value.status(),
            id: value.id(),
        }
    }
}

impl From<UserProfile> for CreateUserProfileInput {
    fn from(value: UserProfile) -> Self {
        CreateUserProfileInput {
            first_name: value.first_name(),
            last_name: value.last_name(),
            bio: value.bio(),
            phone_numbers: value.phone_numbers(),
            avatar_url: value.avatar_url(),
            date_of_birth: value.date_of_birth(),
            addresses: value.addresses(),
            website: value.website(),
            password: value.password(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

impl From<UserPreferences> for CreateUserPreferencesInput {
    fn from(value: UserPreferences) -> Self {
        CreateUserPreferencesInput {
            email_notifications: value.email_notifications(),
            push_notifications: value.push_notifications(),
            two_factor_auth: value.two_factor_auth(),
            language: value.language(),
        }
    }
}

/// Mapper from DTO

impl TryFrom<CreateUserInput> for User {
    type Error = ApplicationError;

    fn try_from(value: CreateUserInput) -> Result<Self, Self::Error> {
        let mut builder = User::new(value.id);
        builder
            .set_email(value.email)
            .set_profile(UserProfile::try_from(value.profile)?)
            .set_username(value.username)
            .set_preferences(UserPreferences::from(value.preferences))
            .add_permissions(value.permissions)
            .add_roles(value.roles)
            .set_status(value.status);
        let user = builder.build()?;
        Ok(user)
    }
}

impl TryFrom<CreateUserProfileInput> for UserProfile {
    type Error = ApplicationError;

    fn try_from(value: CreateUserProfileInput) -> Result<Self, Self::Error> {
        let mut builder = UserProfile::new();
        builder
            .set_avatar_url(value.avatar_url.unwrap_or_default())
            .set_bio(value.bio.unwrap_or_default())
            .add_addresss(value.addresses)
            .add_phone_numbers(value.phone_numbers)
            .set_date_of_birth(value.date_of_birth.unwrap_or_default())
            .set_first_name(value.first_name)
            .set_last_name(value.last_name)
            .set_password(value.password);
        let user_profile = builder.build(value.created_at, value.updated_at)?;
        Ok(user_profile)
    }
}

impl From<CreateUserPreferencesInput> for UserPreferences {
    fn from(value: CreateUserPreferencesInput) -> Self {
        Self::new(
            value.email_notifications,
            value.push_notifications,
            value.two_factor_auth,
            value.language,
        )
    }
}
