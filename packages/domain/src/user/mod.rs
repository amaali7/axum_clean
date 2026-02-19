pub mod preferences;
pub mod profile;

pub use preferences::UserPreferences;
pub use profile::UserProfile;

use crate::error::DomainResult;
use crate::events::DomainEventId;
use crate::{DateTime, DomainError, Email, Event, Permission, RoleId, Username};

use std::collections::HashSet;
use std::ops::DerefMut;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
    pub fn id(&self) -> String {
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for UserId {
    type Target = String;

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
    roles: HashSet<RoleId>,
    permissions: HashSet<Permission>, // Cached permissions for performance
    preferences: UserPreferences,
    status: UserStatus,
    failed_logins: Option<u64>,
    locked_until: Option<DateTime>,
    last_login: Option<DateTime>,
}

impl User {
    pub fn new(id: UserId) -> UserBuilder {
        UserBuilder::new(id)
    }
    // Basic getters - return references to avoid cloning
    pub fn id(&self) -> UserId {
        self.id.clone()
    }

    pub fn email(&self) -> Email {
        self.email.clone()
    }

    pub fn username(&self) -> Username {
        self.username.clone()
    }

    pub fn profile(&self) -> UserProfile {
        self.profile.clone()
    }

    pub fn preferences(&self) -> UserPreferences {
        self.preferences.clone()
    }

    pub fn status(&self) -> UserStatus {
        self.status
    }

    // Collection getters - return references to avoid cloning
    pub fn roles(&self) -> HashSet<RoleId> {
        self.roles.clone()
    }

    pub fn permissions(&self) -> HashSet<Permission> {
        self.permissions.clone()
    }

    pub fn failed_logins(&self) -> Option<u64> {
        self.failed_logins.clone()
    }

    pub fn locked_until(&self) -> Option<DateTime> {
        self.locked_until.clone()
    }

    pub fn last_login(&self) -> Option<DateTime> {
        self.last_login.clone()
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
    roles: HashSet<RoleId>,
    permissions: HashSet<Permission>, // Cached permissions for performance
    preferences: Option<UserPreferences>,
    status: UserStatus,
    failed_logins: Option<u64>,
    locked_until: Option<DateTime>,
    last_login: Option<DateTime>,
}

impl UserBuilder {
    pub fn new(id: UserId) -> Self {
        Self {
            email: None,
            username: None,
            profile: None,
            roles: HashSet::new(),
            permissions: HashSet::new(),
            id,
            preferences: None,
            status: UserStatus::Inactive,
            failed_logins: None,
            locked_until: None,
            last_login: None,
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
    pub fn set_email(&mut self, email: Email) -> &mut Self {
        self.email = Some(email);
        self
    }
    pub fn add_roles(&mut self, roles: HashSet<RoleId>) -> &mut Self {
        self.roles.extend(roles);
        self
    }
    pub fn add_permissions(&mut self, permissions: HashSet<Permission>) -> &mut Self {
        self.permissions.extend(permissions);
        self
    }
    pub fn add_role(&mut self, role: RoleId) -> &mut Self {
        self.roles.insert(role);
        self
    }
    pub fn add_permission(&mut self, permission: Permission) -> &mut Self {
        self.permissions.insert(permission);
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
                .ok_or(DomainError::ValidationError("Email not found".to_string()))?,
            username: self.username.ok_or(DomainError::ValidationError(
                "Username not found".to_string(),
            ))?,
            profile: self.profile.ok_or(DomainError::ValidationError(
                "Profile not found".to_string(),
            ))?,
            roles: self.roles,
            permissions: self.permissions,
            preferences: self.preferences.unwrap_or_default(),
            status: self.status,
            failed_logins: self.failed_logins,
            locked_until: self.locked_until,
            last_login: self.last_login,
        })
    }
}

impl Event for User {
    fn get_type(&self) -> &str {
        "USER"
    }
}
