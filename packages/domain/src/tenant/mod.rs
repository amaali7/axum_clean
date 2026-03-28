pub mod config;
pub mod environment;
pub mod membership;
pub mod permissions;
pub mod specifications;
pub mod temporary_grant;
pub mod value_objects;
pub use membership::Membership;
pub use permissions::Permission;

use std::ops::DerefMut;

use crate::{DateTime, Description, Event, Name};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TenantId(String);

impl TenantId {
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

impl std::ops::Deref for TenantId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TenantId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Tenant {
    id: TenantId,
    name: Name,
    description: Description,
    created_at: DateTime,
}

impl Tenant {
    pub fn new(id: TenantId, name: Name, description: Description, created_at: DateTime) -> Self {
        Self {
            id,
            name,
            description,
            created_at,
        }
    }

    pub fn id(&self) -> &TenantId {
        &self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
    pub fn description(&self) -> &Description {
        &self.description
    }

    pub fn created_at(&self) -> &DateTime {
        &self.created_at
    }
}

impl Event for Tenant {
    fn get_type(&self) -> &str {
        "TENANT"
    }
}
