use std::collections::HashSet;

use domain::User;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::{InfrastructureDateTime, InfrastructureEmail, InfrastructureUsername},
        InfrastructurePermission, InfrastructureRoleId,
    },
};

use super::{
    InfrastructureUserId, InfrastructureUserPreferences, InfrastructureUserProfile, InfrastructureUserStatus,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureUser {
    id: InfrastructureUserId,
    email: InfrastructureEmail,
    username: InfrastructureUsername,
    profile: InfrastructureUserProfile,
    roles: HashSet<InfrastructureRoleId>,
    permissions: HashSet<InfrastructurePermission>, // Cached permissions for performance
    preferences: InfrastructureUserPreferences,
    status: InfrastructureUserStatus,
    failed_logins: Option<u64>,
    locked_until: Option<InfrastructureDateTime>,
    last_login: Option<InfrastructureDateTime>,
}

impl InfrastructureUser {
    pub fn new(id: InfrastructureUserId) -> InfrastructureUserBuilder {
        InfrastructureUserBuilder::new(id)
    }
    // Basic getters - return references to avoid cloning
    pub fn id(&self) -> InfrastructureUserId {
        self.id.clone()
    }

    pub fn email(&self) -> InfrastructureEmail {
        self.email.clone()
    }

    pub fn username(&self) -> InfrastructureUsername {
        self.username.clone()
    }

    pub fn profile(&self) -> InfrastructureUserProfile {
        self.profile.clone()
    }

    pub fn preferences(&self) -> InfrastructureUserPreferences {
        self.preferences.clone()
    }

    pub fn status(&self) -> InfrastructureUserStatus {
        self.status.clone()
    }

    pub fn roles(&self) -> HashSet<InfrastructureRoleId> {
        self.roles.clone()
    }

    pub fn permissions(&self) -> HashSet<InfrastructurePermission> {
        self.permissions.clone()
    }
    pub fn failed_logins(&self) -> Option<u64> {
        self.failed_logins.clone()
    }

    pub fn locked_until(&self) -> Option<InfrastructureDateTime> {
        self.locked_until.clone()
    }

    pub fn last_login(&self) -> Option<InfrastructureDateTime> {
        self.last_login.clone()
    }
}

// Builder pattern for complex object creation
pub struct InfrastructureUserBuilder {
    id: InfrastructureUserId,
    email: Option<InfrastructureEmail>,
    username: Option<InfrastructureUsername>,
    profile: Option<InfrastructureUserProfile>,
    roles: HashSet<InfrastructureRoleId>,
    permissions: HashSet<InfrastructurePermission>, // Cached permissions for performance
    preferences: Option<InfrastructureUserPreferences>,
    status: InfrastructureUserStatus,
    failed_logins: Option<u64>,
    locked_until: Option<InfrastructureDateTime>,
    last_login: Option<InfrastructureDateTime>,
}

impl InfrastructureUserBuilder {
    pub fn new(id: InfrastructureUserId) -> Self {
        Self {
            email: None,
            username: None,
            profile: None,
            roles: HashSet::new(),
            permissions: HashSet::new(),
            id,
            preferences: None,
            status: InfrastructureUserStatus::Inactive,
            failed_logins: None,
            locked_until: None,
            last_login: None,
        }
    }

    pub fn set_preferences(&mut self, preferences: InfrastructureUserPreferences) -> &mut Self {
        self.preferences = Some(preferences);
        self
    }
    pub fn set_status(&mut self, status: InfrastructureUserStatus) -> &mut Self {
        self.status = status;
        self
    }
    pub fn set_email(&mut self, email: InfrastructureEmail) -> &mut Self {
        self.email = Some(email);
        self
    }
    pub fn add_roles(&mut self, roles: HashSet<InfrastructureRoleId>) -> &mut Self {
        self.roles.extend(roles);
        self
    }
    pub fn add_permissions(&mut self, permissions: HashSet<InfrastructurePermission>) -> &mut Self {
        self.permissions.extend(permissions);
        self
    }
    pub fn add_role(&mut self, role: InfrastructureRoleId) -> &mut Self {
        self.roles.insert(role);
        self
    }
    pub fn add_permission(&mut self, permission: InfrastructurePermission) -> &mut Self {
        self.permissions.insert(permission);
        self
    }
    pub fn set_username(&mut self, username: InfrastructureUsername) -> &mut Self {
        self.username = Some(username);
        self
    }

    pub fn set_profile(&mut self, profile: InfrastructureUserProfile) -> &mut Self {
        self.profile = Some(profile);
        self
    }

    pub fn set_failed_logins(&mut self, failed_logins: u64) -> &mut Self {
        self.failed_logins = Some(failed_logins);
        self
    }

    pub fn set_locked_until(&mut self, datetime: InfrastructureDateTime) -> &mut Self {
        self.locked_until = Some(datetime);
        self
    }

    pub fn set_last_login(&mut self, datetime: InfrastructureDateTime) -> &mut Self {
        self.last_login = Some(datetime);
        self
    }

    pub fn build(self) -> InfrastructureResult<InfrastructureUser> {
        Ok(InfrastructureUser {
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

impl TryFrom<User> for InfrastructureUser {
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

impl TryFrom<InfrastructureUser> for User {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureUser) -> InfrastructureResult<Self> {
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
