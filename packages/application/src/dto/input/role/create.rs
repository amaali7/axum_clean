use std::collections::HashSet;

use domain::{
    events::DomainEventId,
    value_objects::{DateTime, Description},
    Name, Permission, Role, RoleId,
};

use crate::error::ApplicationError;

pub struct CreateRoleInput {
    pub id: RoleId,
    pub name: Name,
    pub description: Description,
    pub permissions: HashSet<Permission>,
    pub is_system_role: bool,
    pub created_at: DateTime,
}

/// Mapper from Domain

impl From<Role> for CreateRoleInput {
    fn from(value: Role) -> Self {
        Self {
            name: value.name(),
            description: value.description(),
            permissions: value.permissions(),
            is_system_role: value.is_system_role(),
            created_at: value.created_at(),
            id: value.id(),
        }
    }
}

/// Mapper from DOT

impl TryFrom<CreateRoleInput> for Role {
    type Error = ApplicationError;
    fn try_from(value: CreateRoleInput) -> Result<Self, Self::Error> {
        let mut builder = Self::new(value.id);
        builder
            .set_name(value.name)
            .set_description(value.description)
            .set_created_at(value.created_at)
            .set_is_system_role(value.is_system_role);
        for permission in value.permissions.into_iter() {
            builder.add_permission(permission);
        }
        let role = builder.build()?;
        Ok(role)
    }
}
