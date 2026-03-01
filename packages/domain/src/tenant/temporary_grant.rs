use std::collections::HashSet;

use crate::{
    value_objects::{Action, Resource},
    DateTime, Description, Event, UserId,
};

use super::Permission;

#[derive(Debug, Clone)]
pub struct TemporaryGrant {
    user_id: UserId,
    description: Description,
    permissions: HashSet<Permission>,
    expires_at: DateTime,
    created_at: DateTime,
}

impl TemporaryGrant {
    pub fn new(
        user_id: UserId,
        permissions: HashSet<Permission>,
        expires_at: DateTime,
        created_at: DateTime,
        description: Description,
    ) -> Self {
        Self {
            user_id,
            permissions,
            expires_at,
            description,
            created_at,
        }
    }

    pub fn has_permission(&self, resource: &Resource, action: &Action) -> bool {
        self.permissions.iter().any(|p| p.matches(resource, action))
    }
    pub fn user_id(&self) -> UserId {
        self.user_id.clone()
    }
    pub fn permissions(&self) -> HashSet<Permission> {
        self.permissions.clone()
    }
    pub fn description(&self) -> Description {
        self.description.clone()
    }

    pub fn created_at(&self) -> DateTime {
        self.created_at.clone()
    }
    pub fn expires_at(&self) -> DateTime {
        self.expires_at.clone()
    }
}

impl Event for TemporaryGrant {
    fn get_type(&self) -> &str {
        "TEMPORARYGRANT"
    }
}
