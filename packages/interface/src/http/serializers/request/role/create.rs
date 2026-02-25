use std::collections::HashSet;

use application::dto::role_dto::input::CreateRoleInput;
use serde::Deserialize;

use crate::{
    common_objects::role::{permissions::InterfacePermission, InterfaceRoleId},
    error::{InterfaceError, InterfaceResult},
    value_objects::{InterfaceDescription, InterfaceName},
};

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRoleRequest {
    pub id: InterfaceRoleId,
    pub name: InterfaceName,
    pub description: InterfaceDescription,
    pub permissions: HashSet<InterfacePermission>,
    pub is_system_role: bool,
}

/// Mapper from Domain

impl TryFrom<CreateRoleRequest> for CreateRoleInput {
    type Error = InterfaceError;
    fn try_from(value: CreateRoleRequest) -> InterfaceResult<Self> {
        Ok(Self {
            name: value.name.try_into()?,
            description: value.description.try_into()?,
            permissions: value
                .permissions
                .into_iter()
                .map(|permission| permission.into())
                .collect(),
            is_system_role: value.is_system_role,
            id: value.id.into(),
        })
    }
}
