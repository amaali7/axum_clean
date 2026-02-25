use std::collections::HashSet;

use domain::{
    value_objects::{DateTime, Description},
    Name, Permission, Role, RoleId,
};

use crate::error::ApplicationError;

pub struct UpdateRoleInput {
    pub id: RoleId,
    pub name: Option<Name>,
    pub description: Option<Description>,
    pub permissions: HashSet<Permission>,
    pub is_system_role: Option<bool>,
}

impl From<Role> for UpdateRoleInput {
    fn from(value: Role) -> Self {
        Self {
            name: Some(value.name()),
            description: Some(value.description()),
            permissions: value.permissions(),
            is_system_role: Some(value.is_system_role()),
            id: value.id(),
        }
    }
}

/// Mapper from DOT

impl TryFrom<UpdateRoleInput> for Role {
    type Error = ApplicationError;
    fn try_from(value: UpdateRoleInput) -> Result<Self, Self::Error> {
        let mut builder = Self::new(value.id);
        builder
            .set_name(value.name.unwrap_or_default())
            .set_description(value.description.unwrap_or_default())
            .set_created_at(DateTime::new(0))
            .set_is_system_role(value.is_system_role.unwrap_or_default());
        for permission in value.permissions.into_iter() {
            builder.add_permission(permission);
        }
        let role = builder.build()?;
        Ok(role)
    }
}
