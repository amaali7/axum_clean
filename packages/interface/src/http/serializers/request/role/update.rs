use std::collections::HashSet;

use application::dto::role_dto::input::UpdateRoleInput;
use serde::Deserialize;

use crate::{
    common_objects::role::{permissions::InterfacePermission, InterfaceRoleId},
    error::{InterfaceError, InterfaceResult},
    value_objects::{InterfaceDescription, InterfaceName},
};

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRoleRequest {
    pub id: InterfaceRoleId,
    pub name: Option<InterfaceName>,
    pub description: Option<InterfaceDescription>,
    pub permissions: HashSet<InterfacePermission>,
    pub is_system_role: Option<bool>,
}

impl TryFrom<UpdateRoleRequest> for UpdateRoleInput {
    type Error = InterfaceError;
    fn try_from(value: UpdateRoleRequest) -> InterfaceResult<Self> {
        Ok(Self {
            name: value.name.map(|a| a.try_into()).transpose()?,
            description: value.description.map(|a| a.try_into()).transpose()?,
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
