pub mod profile;

pub use profile::UserProfile;

use crate::events::DomainEvent;
use crate::{DomainError, Email, Permission, RoleId, Username};

use std::collections::HashSet;
use std::ops::DerefMut;

use std::collections::VecDeque;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse_str(s: &str) -> Result<Self, super::error::UserError> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| super::error::UserError::InvalidUserId(s.to_string()))
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

impl std::ops::Deref for UserId {
    type Target = Uuid;

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

#[derive(Debug)]
pub struct User {
    id: UserId,
    email: Email,
    username: Username,
    profile: UserProfile,
    roles: HashSet<RoleId>,
    permissions: HashSet<Permission>, // Cached permissions for performance
    preferences: UserPreferences,
    status: UserStatus,
    events: VecDeque<Box<dyn DomainEvent>>,
}

impl User {
    pub fn new() -> Result<UserBuilder, super::error::DomainError> {
        Ok(UserBuilder::new())
    }
}

#[derive(Debug, Clone)]
pub struct UserPreferences {
    email_notifications: bool,
    push_notifications: bool,
    two_factor_auth: bool,
    language: String,
    timezone: String,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            email_notifications: true,
            push_notifications: true,
            two_factor_auth: false,
            language: "en".to_string(),
            timezone: "UTC".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Suspended,
    Inactive,
    Banned,
}

// Builder pattern for complex object creation
pub struct UserBuilder {
    email: Option<Email>,
    username: Option<Username>,
    profile: Option<UserProfile>,
    roles: HashSet<RoleId>,
    permissions: HashSet<Permission>, // Cached permissions for performance
}

impl UserBuilder {
    pub fn new() -> Self {
        let roles: HashSet<RoleId> = HashSet::new();
        let permissions: HashSet<Permission> = HashSet::new();
        Self {
            email: None,
            username: None,
            profile: None,
            roles,
            permissions,
        }
    }

    pub fn set_email(mut self, email: Email) -> Self {
        self.email = Some(email);
        self
    }

    pub fn add_role(mut self, role: RoleId) -> Self {
        self.roles.insert(role);
        self
    }
    pub fn add_permission(mut self, permission: Permission) -> Self {
        self.permissions.insert(permission);
        self
    }
    pub fn set_username(mut self, username: Username) -> Self {
        self.username = Some(username);
        self
    }

    pub fn set_profile(mut self, profile: UserProfile) -> Self {
        self.profile = Some(profile);
        self
    }

    pub fn build(self) -> Result<User, super::error::DomainError> {
        let events: VecDeque<Box<dyn DomainEvent>> = VecDeque::new();
        Ok(User {
            id: UserId::new(),
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
            preferences: UserPreferences::default(),
            status: UserStatus::Inactive,
            events,
        })
    }
}
