use crate::{
    value_objects::{Action, Resource},
    DateTime, Description, Event,
};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Permission {
    resource: Resource,
    action: Action,
    description: Description,
    created_at: DateTime,
}

impl Permission {
    pub fn new(
        resource: Resource,
        action: Action,
        description: Description,
        created_at: DateTime,
    ) -> Self {
        Self {
            resource,
            action,
            description,
            created_at,
        }
    }

    pub fn matches(&self, resource: &Resource, action: &Action) -> bool {
        &self.resource == resource && &self.action == action
    }
    pub fn resource(&self) -> &Resource {
        &self.resource
    }

    pub fn action(&self) -> &Action {
        &self.action
    }
    pub fn description(&self) -> &Description {
        &self.description
    }

    pub fn created_at(&self) -> &DateTime {
        &self.created_at
    }
}

impl Event for Permission {
    fn get_type(&self) -> &str {
        "PERMISSION"
    }
}
