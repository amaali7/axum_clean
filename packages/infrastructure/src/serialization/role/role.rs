use std::collections::HashSet;

use domain::Role;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::value_objects::{InfrastructureDateTime, InfrastructureDescription, InfrastructureName},
};

use super::{InfrastructurePermission, InfrastructureRoleId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureRole {
    id: InfrastructureRoleId,
    name: InfrastructureName,
    description: InfrastructureDescription,
    permissions: HashSet<InfrastructurePermission>,
    is_system_role: bool,
    created_at: InfrastructureDateTime,
}

impl InfrastructureRole {
    pub fn new(id: &str) -> InfrastructureRoleBuilder {
        InfrastructureRoleBuilder::new(id)
    }

    pub fn id(&self) -> InfrastructureRoleId {
        self.id.clone()
    }

    pub fn name(&self) -> InfrastructureName {
        self.name.clone()
    }
    pub fn description(&self) -> InfrastructureDescription {
        self.description.clone()
    }

    pub fn permissions(&self) -> HashSet<InfrastructurePermission> {
        self.permissions.clone()
    }

    pub fn is_system_role(&self) -> bool {
        self.is_system_role.clone()
    }

    pub fn created_at(&self) -> InfrastructureDateTime {
        self.created_at.clone()
    }
}

#[derive(Debug, Clone)]
pub struct InfrastructureRoleBuilder {
    id: InfrastructureRoleId,
    name: Option<InfrastructureName>,
    description: Option<InfrastructureDescription>,
    permissions: HashSet<InfrastructurePermission>,
    is_system_role: Option<bool>,
    created_at: Option<InfrastructureDateTime>,
}

impl InfrastructureRoleBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: InfrastructureRoleId::new(id),
            name: None,
            description: None,
            permissions: HashSet::new(),
            is_system_role: None,
            created_at: None,
        }
    }

    pub fn set_name(&mut self, name: InfrastructureName) -> &mut Self {
        self.name = Some(name);
        self
    }
    pub fn set_description(&mut self, description: InfrastructureDescription) -> &mut Self {
        self.description = Some(description);
        self
    }

    pub fn add_permission(&mut self, permission: InfrastructurePermission) -> &mut Self {
        self.permissions.insert(permission);
        self
    }

    pub fn set_is_system_role(&mut self, is_system_role: bool) -> &mut Self {
        self.is_system_role = Some(is_system_role);
        self
    }

    pub fn set_created_at(&mut self, created_at: InfrastructureDateTime) -> &mut Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn build(self) -> InfrastructureResult<InfrastructureRole> {
        Ok(InfrastructureRole {
            id: self.id,
            name: self.name.ok_or(InfrastructureError::ValidationError(
                "Name not found".to_string(),
            ))?,
            description: self
                .description
                .ok_or(InfrastructureError::ValidationError(
                    "InfrastructureDescription not found".to_string(),
                ))?,
            permissions: self.permissions,
            is_system_role: self
                .is_system_role
                .ok_or(InfrastructureError::ValidationError(
                    "Is System InfrastructureRole not found".to_string(),
                ))?,
            created_at: self.created_at.ok_or(InfrastructureError::ValidationError(
                "Created At not found".to_string(),
            ))?,
        })
    }
}

impl TryFrom<Role> for InfrastructureRole {
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

impl TryFrom<InfrastructureRole> for Role {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureRole) -> InfrastructureResult<Self> {
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
