use std::collections::HashSet;

use domain::Role;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::value_objects::{SerializedDateTime, SerializedDescription, SerializedName},
};

use super::{SerializedPermission, SerializedRoleId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedRole {
    id: SerializedRoleId,
    name: SerializedName,
    description: SerializedDescription,
    permissions: HashSet<SerializedPermission>,
    is_system_role: bool,
    created_at: SerializedDateTime,
}

impl SerializedRole {
    pub fn new(id: &str) -> SerializedRoleBuilder {
        SerializedRoleBuilder::new(id)
    }

    pub fn id(&self) -> SerializedRoleId {
        self.id.clone()
    }

    pub fn name(&self) -> SerializedName {
        self.name.clone()
    }
    pub fn description(&self) -> SerializedDescription {
        self.description.clone()
    }

    pub fn permissions(&self) -> HashSet<SerializedPermission> {
        self.permissions.clone()
    }

    pub fn is_system_role(&self) -> bool {
        self.is_system_role.clone()
    }

    pub fn created_at(&self) -> SerializedDateTime {
        self.created_at.clone()
    }
}

#[derive(Debug, Clone)]
pub struct SerializedRoleBuilder {
    id: SerializedRoleId,
    name: Option<SerializedName>,
    description: Option<SerializedDescription>,
    permissions: HashSet<SerializedPermission>,
    is_system_role: Option<bool>,
    created_at: Option<SerializedDateTime>,
}

impl SerializedRoleBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: SerializedRoleId::new(id),
            name: None,
            description: None,
            permissions: HashSet::new(),
            is_system_role: None,
            created_at: None,
        }
    }

    pub fn set_name(&mut self, name: SerializedName) -> &mut Self {
        self.name = Some(name);
        self
    }
    pub fn set_description(&mut self, description: SerializedDescription) -> &mut Self {
        self.description = Some(description);
        self
    }

    pub fn add_permission(&mut self, permission: SerializedPermission) -> &mut Self {
        self.permissions.insert(permission);
        self
    }

    pub fn set_is_system_role(&mut self, is_system_role: bool) -> &mut Self {
        self.is_system_role = Some(is_system_role);
        self
    }

    pub fn set_created_at(&mut self, created_at: SerializedDateTime) -> &mut Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn build(self) -> InfrastructureResult<SerializedRole> {
        Ok(SerializedRole {
            id: self.id,
            name: self.name.ok_or(InfrastructureError::ValidationError(
                "Name not found".to_string(),
            ))?,
            description: self
                .description
                .ok_or(InfrastructureError::ValidationError(
                    "SerializedDescription not found".to_string(),
                ))?,
            permissions: self.permissions,
            is_system_role: self
                .is_system_role
                .ok_or(InfrastructureError::ValidationError(
                    "Is System SerializedRole not found".to_string(),
                ))?,
            created_at: self.created_at.ok_or(InfrastructureError::ValidationError(
                "Created At not found".to_string(),
            ))?,
        })
    }
}

impl TryFrom<Role> for SerializedRole {
    type Error = InfrastructureError;

    fn try_from(value: Role) -> InfrastructureResult<Self> {
        let mut role_builder = Self::new(&value.id().id());
        role_builder
            .set_name(value.name().try_into()?)
            .set_description(value.description().try_into()?)
            .set_created_at(value.created_at().try_into()?)
            .set_is_system_role(value.is_system_role());
        for permission in value.permissions().into_iter() {
            role_builder.add_permission(permission.into());
        }
        let role = role_builder.build()?;
        Ok(role.clone())
    }
}

impl TryFrom<SerializedRole> for Role {
    type Error = InfrastructureError;

    fn try_from(value: SerializedRole) -> InfrastructureResult<Self> {
        let mut role_builder = Self::new(value.id().into());
        role_builder
            .set_name(value.name().try_into()?)
            .set_description(value.description().try_into()?)
            .set_created_at(value.created_at().try_into()?)
            .set_is_system_role(value.is_system_role());
        for permission in value.permissions().into_iter() {
            role_builder.add_permission(permission.into());
        }
        let role = role_builder.build()?;
        Ok(role.clone())
    }
}
