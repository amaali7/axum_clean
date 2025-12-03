use std::collections::HashSet;

use domain::{
    events::DomainEventId,
    value_objects::{DateTime, Description},
    Name, Permission, Role, RoleId,
};

pub struct PrivilegeRoleOutput {
    pub id: RoleId,
    pub name: Name,
    pub description: Description,
    pub permissions: HashSet<Permission>,
    pub is_system_role: bool,
    pub created_at: DateTime,
    pub events: Vec<DomainEventId>,
}

impl From<Role> for PrivilegeRoleOutput {
    fn from(value: Role) -> Self {
        Self {
            id: value.id(),
            name: value.name(),
            description: value.description(),
            permissions: value.permissions(),
            is_system_role: value.is_system_role(),
            created_at: value.created_at(),
            events: value.events(),
        }
    }
}
