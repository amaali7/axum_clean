pub mod permissions;

pub use permissions::Permission;

use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Role {
    id: RoleId,
    name: String,
    description: String,
    permissions: HashSet<Permission>,
    is_system_role: bool,
    created_at: String,
}

impl Role {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: RoleId::new(),
            name,
            description,
            permissions: HashSet::new(),
            is_system_role: false,
            created_at: String::new(),
        }
    }

    pub fn add_permission(&mut self, permission: Permission) -> bool {
        self.permissions.insert(permission)
    }

    pub fn remove_permission(&mut self, permission: &Permission) -> bool {
        self.permissions.remove(permission)
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions.contains(permission)
    }

    pub fn can_grant_permission(&self, permission: &Permission) -> bool {
        // System roles can grant any permission, regular roles have limits
        self.is_system_role || !permission.is_sensitive()
    }

    // Getters
    pub fn id(&self) -> &RoleId {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn permissions(&self) -> &HashSet<Permission> {
        &self.permissions
    }
    pub fn is_system_role(&self) -> bool {
        self.is_system_role
    }
}
