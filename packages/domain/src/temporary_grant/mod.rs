use std::collections::HashSet;

use crate::{
    value_objects::{Action, Resource},
    DateTime, Description, Event, UserId,
};

#[derive(Debug, Clone)]
pub struct TemporaryGrant {
    user_id: UserId,
    description: Description,
    resource: Resource,
    action: Action,
    expires_at: DateTime,
    created_at: DateTime,
    version: u64,
}

#[derive(Debug, Clone)]
pub struct TemporaryGrantParts {
    pub user_id: UserId,
    pub description: Description,
    pub resource: Resource,
    pub action: Action,
    pub expires_at: DateTime,
    pub created_at: DateTime,
    pub version: u64,
}

impl TemporaryGrant {
    pub fn new(
        user_id: UserId,
        description: Description,
        resource: Resource,
        action: Action,
        expires_at: DateTime,
        created_at: DateTime,
        version: u64,
    ) -> Self {
        Self {
            user_id,
            expires_at,
            description,
            created_at,
            resource,
            action,
            version,
        }
    }

    pub fn into_parts(self) -> TemporaryGrantParts {
        let Self {
            user_id,
            description,
            resource,
            action,
            expires_at,
            created_at,
            version,
        } = self;
        TemporaryGrantParts {
            user_id,
            description,
            resource,
            action,
            expires_at,
            created_at,
            version,
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
    pub fn version(&self) -> &u64 {
        &self.version
    }
}

impl Event for TemporaryGrant {
    fn get_type(&self) -> &str {
        "TEMPORARYGRANT"
    }
}
