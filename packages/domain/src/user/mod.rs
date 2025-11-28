pub mod profile;

pub use profile::UserProfile;

use crate::events::DomainEvent;
use crate::{DomainError, Email, Permission, RoleId, Username};

use std::collections::HashSet;
use std::ops::DerefMut;

use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new() -> Self {
        Self(String::new())
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
    events: VecDeque<DomainEvent>,
}

impl User {
    pub fn new(id: UserId) -> Result<UserBuilder, super::error::DomainError> {
        Ok(UserBuilder::new(id))
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
        self.status.clone()
    }

    // Collection getters - return references to avoid cloning
    pub fn roles(&self) -> HashSet<RoleId> {
        self.roles.clone()
    }

    pub fn permissions(&self) -> HashSet<Permission> {
        self.permissions.clone()
    }

    pub fn events(&self) -> VecDeque<DomainEvent> {
        self.events.clone()
    }
}

#[derive(Debug, Clone)]
pub struct UserPreferences {
    email_notifications: bool,
    push_notifications: bool,
    two_factor_auth: bool,
    language: String,
}

impl UserPreferences {
    pub fn email_notifications(&self) -> bool {
        self.email_notifications.clone()
    }

    pub fn push_notifications(&self) -> bool {
        self.push_notifications.clone()
    }

    pub fn two_factor_auth(&self) -> bool {
        self.two_factor_auth.clone()
    }

    pub fn language(&self) -> String {
        self.language.clone()
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            email_notifications: true,
            push_notifications: true,
            two_factor_auth: false,
            language: "en".to_string(),
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
    id: UserId,
    email: Option<Email>,
    username: Option<Username>,
    profile: Option<UserProfile>,
    roles: HashSet<RoleId>,
    permissions: HashSet<Permission>, // Cached permissions for performance
    events: VecDeque<DomainEvent>,
}

impl UserBuilder {
    pub fn new(id: UserId) -> Self {
        let roles: HashSet<RoleId> = HashSet::new();
        let permissions: HashSet<Permission> = HashSet::new();
        let events: VecDeque<DomainEvent> = VecDeque::new();
        Self {
            email: None,
            username: None,
            profile: None,
            roles,
            permissions,
            events,
            id,
        }
    }

    pub fn set_email(mut self, email: Email) -> Self {
        self.email = Some(email);
        self
    }
    pub fn add_roles(mut self, roles: HashSet<RoleId>) -> Self {
        self.roles.extend(roles);
        self
    }
    pub fn add_permissions(mut self, permissions: HashSet<Permission>) -> Self {
        self.permissions.extend(permissions);
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
    fn add_event(mut self, event: DomainEvent) -> Self {
        self.events.push_back(event);
        self
    }

    pub fn build(self) -> Result<User, super::error::DomainError> {
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
            preferences: UserPreferences::default(),
            status: UserStatus::Inactive,
            events: self.events,
        })
    }
}
