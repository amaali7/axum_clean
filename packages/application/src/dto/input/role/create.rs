use std::collections::HashSet;

use domain::{
    events::DomainEventId,
    value_objects::{DateTime, Description},
    Name, Permission, Role, RoleId,
};

pub struct CreateRoleInput {
    pub id: RoleId,
    pub name: Name,
    pub description: Description,
    pub permissions: HashSet<Permission>,
    pub is_system_role: bool,
    pub created_at: DateTime,
    pub events: Vec<DomainEventId>,
}

impl From<Role> for CreateRoleInput {
    fn from(value: Role) -> Self {
        Self {
            name: value.name(),
            description: value.description(),
            permissions: value.permissions(),
            is_system_role: value.is_system_role(),
            created_at: value.created_at(),
            id: value.id(),
            events: value.events(),
        }
    }
}
