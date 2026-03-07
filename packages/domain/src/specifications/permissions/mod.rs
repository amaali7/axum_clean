use crate::value_objects::{Action, Resource};

pub struct HasPermission {
    resource: Resource,
    action: Action,
}

impl HasPermission {
    pub fn new(resource: Resource, action: Action) -> Self {
        Self { resource, action }
    }
}
