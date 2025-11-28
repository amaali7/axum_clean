pub mod permissions;

pub use permissions::Permission;

use std::collections::{HashSet, VecDeque};
use std::ops::{Deref, DerefMut};

use crate::events::DomainEvent;
use crate::value_objects::{DateTime, Description};
use crate::{DomainError, Name};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoleId(String);

impl RoleId {
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl Deref for RoleId {
    type Target = String;
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
    permissions: HashSet<Permission>,
    is_system_role: bool,
    created_at: DateTime,
    events: VecDeque<DomainEvent>,
}

impl Role {
    pub fn new(id: RoleId) -> RoleBuilder {
        RoleBuilder::new(id)
    }

    pub fn id(&self) -> RoleId {
        self.id.clone()
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }
    pub fn description(&self) -> Description {
        self.description.clone()
    }

    pub fn permissions(&self) -> HashSet<Permission> {
        self.permissions.clone()
    }

    pub fn is_system_role(&self) -> bool {
        self.is_system_role.clone()
    }

    pub fn created_at(&self) -> DateTime {
        self.created_at.clone()
    }

    pub fn events(&self) -> VecDeque<DomainEvent> {
        self.events.clone()
    }
}

#[derive(Debug, Clone)]
pub struct RoleBuilder {
    id: RoleId,
    name: Option<Name>,
    description: Option<Description>,
    permissions: HashSet<Permission>,
    is_system_role: Option<bool>,
    created_at: Option<DateTime>,
    events: VecDeque<DomainEvent>,
}

impl RoleBuilder {
    pub fn new(id: RoleId) -> Self {
        let events: VecDeque<DomainEvent> = VecDeque::new();
        Self {
            id,
            name: None,
            description: None,
            permissions: HashSet::new(),
            is_system_role: None,
            created_at: None,
            events,
        }
    }

    pub fn set_name(mut self, name: Name) -> Self {
        self.name = Some(name);
        self
    }
    pub fn set_description(mut self, description: Description) -> Self {
        self.description = Some(description);
        self
    }

    pub fn add_permissions(mut self, permission: Permission) -> Self {
        self.permissions.insert(permission);
        self
    }

    pub fn set_is_system_role(mut self, is_system_role: bool) -> Self {
        self.is_system_role = Some(is_system_role);
        self
    }

    pub fn set_created_at(mut self, created_at: DateTime) -> Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn events(mut self, event: DomainEvent) -> Self {
        self.events.insert(self.events.len() + 1, event);
        self
    }

    pub fn build(self) -> Result<Role, DomainError> {
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
            events: self.events,
        })
    }
}
