pub mod fields;
pub mod preferences;
pub mod profile;

pub use preferences::UserPreferences;
pub use profile::UserProfile;

use crate::error::DomainResult;
use crate::{DateTime, DomainError, Email, Event, Username};

use std::ops::DerefMut;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: &str) -> Self {
        Self(id.into())
    }
    pub fn id(&self) -> &str {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for UserId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for UserId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone)]
pub struct User {
    id: UserId,
    email: Email,
    username: Username,
    profile: UserProfile,
    preferences: UserPreferences,
    status: UserStatus,
    failed_logins: Option<u64>,
    locked_until: Option<DateTime>,
    last_login: Option<DateTime>,
    version: u64,
}
#[derive(Debug, Default, Clone)]
pub struct UserParts {
    pub id: UserId,
    pub email: Email,
    pub username: Username,
    pub profile: UserProfile,
    pub preferences: UserPreferences,
    pub status: UserStatus,
    pub failed_logins: Option<u64>,
    pub locked_until: Option<DateTime>,
    pub last_login: Option<DateTime>,
    pub version: u64,
}

impl User {
    pub fn new(id: UserId) -> UserBuilder {
        UserBuilder::new(id)
    }
    pub fn is_active(&self) -> bool {
        self.status == UserStatus::Active
    }
    // into parts
    pub fn into_parts(self) -> UserParts {
        let User {
            id,
            email,
            username,
            profile,
            preferences,
            status,
            failed_logins,
            locked_until,
            last_login,
            version,
        } = self;
        UserParts {
            id,
            email,
            username,
            profile,
            preferences,
            status,
            failed_logins,
            locked_until,
            last_login,
            version,
        }
    }
    // Basic getters - return references to avoid cloning
    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn profile(&self) -> &UserProfile {
        &self.profile
    }

    pub fn preferences(&self) -> &UserPreferences {
        &self.preferences
    }

    pub fn status(&self) -> &UserStatus {
        &self.status
    }

    // Collection getters - return references to avoid cloning
    pub fn failed_logins(&self) -> &Option<u64> {
        &self.failed_logins
    }

    pub fn locked_until(&self) -> &Option<DateTime> {
        &self.locked_until
    }

    pub fn last_login(&self) -> &Option<DateTime> {
        &self.last_login
    }

    pub fn version(&self) -> &u64 {
        &self.version
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UserStatus {
    Active,
    Suspended,
    #[default]
    Inactive,
    Banned,
}

// Builder pattern for complex object creation
pub struct UserBuilder {
    id: UserId,
    email: Option<Email>,
    username: Option<Username>,
    profile: Option<UserProfile>,
    preferences: Option<UserPreferences>,
    status: UserStatus,
    failed_logins: Option<u64>,
    locked_until: Option<DateTime>,
    last_login: Option<DateTime>,
    version: u64,
}

impl UserBuilder {
    pub fn new(id: UserId) -> Self {
        Self {
            email: None,
            username: None,
            profile: None,
            id,
            preferences: None,
            status: UserStatus::Inactive,
            failed_logins: None,
            locked_until: None,
            last_login: None,
            version: 0,
        }
    }

    pub fn set_preferences(&mut self, preferences: UserPreferences) -> &mut Self {
        self.preferences = Some(preferences);
        self
    }
    pub fn set_status(&mut self, status: UserStatus) -> &mut Self {
        self.status = status;
        self
    }
    pub fn set_version(&mut self, version: u64) -> &mut Self {
        self.version = version;
        self
    }
    pub fn set_email(&mut self, email: Email) -> &mut Self {
        self.email = Some(email);
        self
    }
    pub fn set_username(&mut self, username: Username) -> &mut Self {
        self.username = Some(username);
        self
    }

    pub fn set_profile(&mut self, profile: UserProfile) -> &mut Self {
        self.profile = Some(profile);
        self
    }

    pub fn set_failed_logins(&mut self, failed_logins: u64) -> &mut Self {
        self.failed_logins = Some(failed_logins);
        self
    }

    pub fn set_locked_until(&mut self, datetime: DateTime) -> &mut Self {
        self.locked_until = Some(datetime);
        self
    }

    pub fn set_last_login(&mut self, datetime: DateTime) -> &mut Self {
        self.last_login = Some(datetime);
        self
    }

    pub fn build(self) -> DomainResult<User> {
        Ok(User {
            id: self.id,
            email: self
                .email
                .ok_or(DomainError::ValidationError("Email not found".into()))?,
            username: self
                .username
                .ok_or(DomainError::ValidationError("Username not found".into()))?,
            profile: self
                .profile
                .ok_or(DomainError::ValidationError("Profile not found".into()))?,
            preferences: self.preferences.unwrap_or_default(),
            status: self.status,
            failed_logins: self.failed_logins,
            locked_until: self.locked_until,
            last_login: self.last_login,
            version: self.version,
        })
    }
}

impl Event for User {
    fn get_type(&self) -> &str {
        "USER"
    }
}
