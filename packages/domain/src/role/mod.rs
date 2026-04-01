pub mod permissions;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

pub use permissions::{Permission, PermissionId, PermissionParts};

use crate::error::DomainResult;
use crate::value_objects::{Action, DateTime, Description, Resource};
use crate::{DomainError, Event, Name};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoleId(String);

impl RoleId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
    pub fn id(&self) -> &String {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for RoleId {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RoleId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct Role {
    id: RoleId,
    name: Name,
    description: Description,
    permissions: HashSet<PermissionId>,
    is_system_role: bool,
    created_at: DateTime,
    version: u64,
}
#[derive(Debug, Clone)]
pub struct RoleParts {
    pub id: RoleId,
    pub name: Name,
    pub description: Description,
    pub permissions: HashSet<PermissionId>,
    pub is_system_role: bool,
    pub created_at: DateTime,
    pub version: u64,
}

impl Role {
    pub fn new(id: RoleId) -> RoleBuilder {
        RoleBuilder::new(id)
    }

    pub fn into_parts(self) -> RoleParts {
        let Self {
            id,
            name,
            description,
            permissions,
            is_system_role,
            created_at,
            version,
        } = self;
        RoleParts {
            id,
            name,
            description,
            permissions,
            is_system_role,
            created_at,
            version,
        }
    }

    pub fn id(&self) -> &RoleId {
        &self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn description(&self) -> &Description {
        &self.description
    }

    pub fn permissions(&self) -> &HashSet<PermissionId> {
        &self.permissions
    }

    pub fn is_system_role(&self) -> bool {
        self.is_system_role
    }

    pub fn created_at(&self) -> &DateTime {
        &self.created_at
    }
    pub fn version(&self) -> &u64 {
        &self.version
    }
}

#[derive(Debug, Clone)]
pub struct RoleBuilder {
    id: RoleId,
    name: Option<Name>,
    description: Option<Description>,
    permissions: HashSet<PermissionId>,
    is_system_role: Option<bool>,
    created_at: Option<DateTime>,
    version: u64,
}

impl RoleBuilder {
    pub fn new(id: RoleId) -> Self {
        Self {
            id,
            name: None,
            description: None,
            permissions: HashSet::new(),
            is_system_role: None,
            created_at: None,
            version: 0,
        }
    }

    pub fn set_name(&mut self, name: Name) -> &mut Self {
        self.name = Some(name);
        self
    }
    pub fn set_version(&mut self, version: u64) -> &mut Self {
        self.version = version;
        self
    }
    pub fn set_description(&mut self, description: Description) -> &mut Self {
        self.description = Some(description);
        self
    }

    pub fn add_permission(&mut self, permission: PermissionId) -> &mut Self {
        self.permissions.insert(permission);
        self
    }

    pub fn set_is_system_role(&mut self, is_system_role: bool) -> &mut Self {
        self.is_system_role = Some(is_system_role);
        self
    }

    pub fn set_created_at(&mut self, created_at: DateTime) -> &mut Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn build(self) -> DomainResult<Role> {
        Ok(Role {
            id: self.id,
            name: self
                .name
                .ok_or(DomainError::ValidationError("Name not found".to_string()))?,
            description: self.description.ok_or(DomainError::ValidationError(
                "Description not found".to_string(),
            ))?,
            permissions: self.permissions,
            is_system_role: self.is_system_role.ok_or(DomainError::ValidationError(
                "Is System Role not found".to_string(),
            ))?,
            created_at: self.created_at.ok_or(DomainError::ValidationError(
                "Created At not found".to_string(),
            ))?,
            version: self.version,
        })
    }
}

impl Event for Role {
    fn get_type(&self) -> &str {
        "ROLE"
    }
}
