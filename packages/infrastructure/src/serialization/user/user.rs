use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        events::SerializedEventId,
        value_objects::{SerializedEmail, SerializedUsername},
        SerializedPermission, SerializedRoleId,
    },
};

use super::{
    SerializedUserId, SerializedUserPreferences, SerializedUserProfile, SerializedUserStatus,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedUser {
    id: SerializedUserId,
    email: SerializedEmail,
    username: SerializedUsername,
    profile: SerializedUserProfile,
    roles: HashSet<SerializedRoleId>,
    permissions: HashSet<SerializedPermission>, // Cached permissions for performance
    preferences: SerializedUserPreferences,
    status: SerializedUserStatus,
    events: Vec<SerializedEventId>,
}

impl SerializedUser {
    pub fn new(id: SerializedUserId) -> SerializedUserBuilder {
        SerializedUserBuilder::new(id)
    }
    // Basic getters - return references to avoid cloning
    pub fn id(&self) -> SerializedUserId {
        self.id.clone()
    }

    pub fn email(&self) -> SerializedEmail {
        self.email.clone()
    }

    pub fn username(&self) -> SerializedUsername {
        self.username.clone()
    }

    pub fn profile(&self) -> SerializedUserProfile {
        self.profile.clone()
    }

    pub fn preferences(&self) -> SerializedUserPreferences {
        self.preferences.clone()
    }

    pub fn status(&self) -> SerializedUserStatus {
        self.status.clone()
    }

    pub fn roles(&self) -> HashSet<SerializedRoleId> {
        self.roles.clone()
    }

    pub fn permissions(&self) -> HashSet<SerializedPermission> {
        self.permissions.clone()
    }

    pub fn events(&self) -> Vec<SerializedEventId> {
        self.events.clone()
    }
}

// Builder pattern for complex object creation
pub struct SerializedUserBuilder {
    id: SerializedUserId,
    email: Option<SerializedEmail>,
    username: Option<SerializedUsername>,
    profile: Option<SerializedUserProfile>,
    roles: HashSet<SerializedRoleId>,
    permissions: HashSet<SerializedPermission>, // Cached permissions for performance
    events: Vec<SerializedEventId>,
    preferences: Option<SerializedUserPreferences>,
    status: SerializedUserStatus,
}

impl SerializedUserBuilder {
    pub fn new(id: SerializedUserId) -> Self {
        Self {
            email: None,
            username: None,
            profile: None,
            roles: HashSet::new(),
            permissions: HashSet::new(),
            events: Vec::new(),
            id,
            preferences: None,
            status: SerializedUserStatus::Inactive,
        }
    }

    pub fn set_preferences(&mut self, preferences: SerializedUserPreferences) -> &mut Self {
        self.preferences = Some(preferences);
        self
    }
    pub fn set_status(&mut self, status: SerializedUserStatus) -> &mut Self {
        self.status = status;
        self
    }
    pub fn set_email(&mut self, email: SerializedEmail) -> &mut Self {
        self.email = Some(email);
        self
    }
    pub fn add_roles(&mut self, roles: HashSet<SerializedRoleId>) -> &mut Self {
        self.roles.extend(roles);
        self
    }
    pub fn add_permissions(&mut self, permissions: HashSet<SerializedPermission>) -> &mut Self {
        self.permissions.extend(permissions);
        self
    }
    pub fn add_role(&mut self, role: SerializedRoleId) -> &mut Self {
        self.roles.insert(role);
        self
    }
    pub fn add_permission(&mut self, permission: SerializedPermission) -> &mut Self {
        self.permissions.insert(permission);
        self
    }
    pub fn set_username(&mut self, username: SerializedUsername) -> &mut Self {
        self.username = Some(username);
        self
    }

    pub fn set_profile(&mut self, profile: SerializedUserProfile) -> &mut Self {
        self.profile = Some(profile);
        self
    }
    pub fn add_event(&mut self, event: SerializedEventId) -> &mut Self {
        self.events.push(event);
        self
    }

    pub fn build(self) -> InfrastructureResult<SerializedUser> {
        Ok(SerializedUser {
            id: self.id,
            email: self.email.ok_or(InfrastructureError::ValidationError(
                "Email not found".to_string(),
            ))?,
            username: self.username.ok_or(InfrastructureError::ValidationError(
                "Username not found".to_string(),
            ))?,
            profile: self.profile.ok_or(InfrastructureError::ValidationError(
                "Profile not found".to_string(),
            ))?,
            roles: self.roles,
            permissions: self.permissions,
            preferences: self.preferences.unwrap_or_default(),
            status: self.status,
            events: self.events,
        })
    }
}
