use std::collections::HashSet;

use domain::User;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::{SerializedDateTime, SerializedEmail, SerializedUsername},
        SerializedPermission, SerializedRoleId,
    },
};

use super::{
    SerializedUserId, SerializedUserPreferences, SerializedUserProfile, SerializedUserStatus,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedUser {
    id: SerializedUserId,
    email: SerializedEmail,
    username: SerializedUsername,
    profile: SerializedUserProfile,
    roles: HashSet<SerializedRoleId>,
    permissions: HashSet<SerializedPermission>, // Cached permissions for performance
    preferences: SerializedUserPreferences,
    status: SerializedUserStatus,
    failed_logins: Option<u64>,
    locked_until: Option<SerializedDateTime>,
    last_login: Option<SerializedDateTime>,
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
    pub fn failed_logins(&self) -> Option<u64> {
        self.failed_logins.clone()
    }

    pub fn locked_until(&self) -> Option<SerializedDateTime> {
        self.locked_until.clone()
    }

    pub fn last_login(&self) -> Option<SerializedDateTime> {
        self.last_login.clone()
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
    preferences: Option<SerializedUserPreferences>,
    status: SerializedUserStatus,
    failed_logins: Option<u64>,
    locked_until: Option<SerializedDateTime>,
    last_login: Option<SerializedDateTime>,
}

impl SerializedUserBuilder {
    pub fn new(id: SerializedUserId) -> Self {
        Self {
            email: None,
            username: None,
            profile: None,
            roles: HashSet::new(),
            permissions: HashSet::new(),
            id,
            preferences: None,
            status: SerializedUserStatus::Inactive,
            failed_logins: None,
            locked_until: None,
            last_login: None,
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

    pub fn set_failed_logins(&mut self, failed_logins: u64) -> &mut Self {
        self.failed_logins = Some(failed_logins);
        self
    }

    pub fn set_locked_until(&mut self, datetime: SerializedDateTime) -> &mut Self {
        self.locked_until = Some(datetime);
        self
    }

    pub fn set_last_login(&mut self, datetime: SerializedDateTime) -> &mut Self {
        self.last_login = Some(datetime);
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
            failed_logins: self.failed_logins,
            locked_until: self.locked_until,
            last_login: self.last_login,
        })
    }
}

impl TryFrom<User> for SerializedUser {
    type Error = InfrastructureError;

    fn try_from(value: User) -> InfrastructureResult<Self> {
        let mut user_builder = Self::new(value.id().into());
        user_builder
            .set_email(value.email().try_into()?)
            .set_status(value.status().into())
            .set_profile(value.profile().try_into()?)
            .set_preferences(value.preferences().try_into()?)
            .set_username(value.username().try_into()?);
        for permission in value.permissions().into_iter() {
            user_builder.add_permission(permission.into());
        }
        for role in value.roles().into_iter() {
            user_builder.add_role(role.into());
        }
        match value.failed_logins() {
            Some(failed_logins) => {
                user_builder.set_failed_logins(failed_logins);
            }
            None => (),
        }

        match value.last_login() {
            Some(last_login) => {
                user_builder.set_last_login(last_login.try_into()?);
            }
            None => (),
        }
        match value.locked_until() {
            Some(locked_until) => {
                user_builder.set_locked_until(locked_until.try_into()?);
            }
            None => (),
        }
        user_builder.build()
    }
}

impl TryFrom<SerializedUser> for User {
    type Error = InfrastructureError;

    fn try_from(value: SerializedUser) -> InfrastructureResult<Self> {
        let mut user_builder = Self::new(value.id().into());
        user_builder
            .set_email(value.email().try_into()?)
            .set_status(value.status().into())
            .set_profile(value.profile().try_into()?)
            .set_preferences(value.preferences().try_into()?)
            .set_username(value.username().try_into()?);
        for permission in value.permissions().into_iter() {
            user_builder.add_permission(permission.into());
        }
        for role in value.roles().into_iter() {
            user_builder.add_role(role.into());
        }
        match value.failed_logins() {
            Some(failed_logins) => {
                user_builder.set_failed_logins(failed_logins);
            }
            None => (),
        }

        match value.last_login() {
            Some(last_login) => {
                user_builder.set_last_login(last_login.try_into()?);
            }
            None => (),
        }
        match value.locked_until() {
            Some(locked_until) => {
                user_builder.set_locked_until(locked_until.try_into()?);
            }
            None => (),
        }
        user_builder
            .build()
            .map_err(|err| InfrastructureError::Domain(err))
    }
}
