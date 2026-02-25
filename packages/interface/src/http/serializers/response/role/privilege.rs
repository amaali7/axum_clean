use std::collections::HashSet;

use application::dto::role_dto::output::PrivilegeRoleOutput;

use crate::{
    common_objects::role::{permissions::InterfacePermission, InterfaceRoleId},
    error::{InterfaceError, InterfaceResult},
    value_objects::{InterfaceDateTime, InterfaceDescription, InterfaceName},
};

pub struct PrivilegeRoleResponse {
    pub id: InterfaceRoleId,
    pub name: InterfaceName,
    pub description: InterfaceDescription,
    pub permissions: HashSet<InterfacePermission>,
    pub is_system_role: bool,
    pub created_at: InterfaceDateTime,
}

impl TryFrom<PrivilegeRoleOutput> for PrivilegeRoleResponse {
    type Error = InterfaceError;

    fn try_from(value: PrivilegeRoleOutput) -> InterfaceResult<Self> {
        Ok(Self {
            id: value.id.into(),
            name: value.name.try_into()?,
            description: value.description.try_into()?,
            permissions: value.permissions.into_iter().map(|x| x.into()).collect(),
            is_system_role: value.is_system_role,
            created_at: value.created_at.try_into()?,
        })
    }
}
