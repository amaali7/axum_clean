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
    resource: Resource,
    action: Action,
    expires_at: DateTime,
    created_at: DateTime,
}

impl TemporaryGrant {
    pub fn new(
        user_id: UserId,
        description: Description,
        resource: Resource,
        action: Action,
        expires_at: DateTime,
        created_at: DateTime,
    ) -> Self {
        Self {
            user_id,
            expires_at,
            description,
            created_at,
            resource,
            action,
        }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
    pub fn description(&self) -> &Description {
        &self.description
    }

    pub fn resource(&self) -> &Resource {
        &self.resource
    }
    pub fn action(&self) -> &Action {
        &self.action
    }
    pub fn created_at(&self) -> &DateTime {
        &self.created_at
    }
    pub fn expires_at(&self) -> &DateTime {
        &self.expires_at
    }
}

impl Event for TemporaryGrant {
    fn get_type(&self) -> &str {
        "TEMPORARYGRANT"
    }
}
