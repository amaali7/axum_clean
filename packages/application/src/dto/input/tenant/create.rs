use std::collections::HashSet;

use domain::{
    tenant::temporary_grant::TemporaryGrant,
    value_objects::{Action, DateTime, Description, Resource},
    Membership, Name, Permission, RoleId, Tenant, TenantId, UserId,
};

pub struct CreateTenantInput {
    pub id: TenantId,
    pub name: Name,
    pub description: Description,
}

/// Mapper from DOT

impl From<CreateTenantInput> for Tenant {
    fn from(value: CreateTenantInput) -> Self {
        Self::new(value.id, value.name, value.description, DateTime::default())
    }
}

/// permission

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct CreatePermissionInput {
    pub resource: Resource,
    pub action: Action,
    pub description: Description,
}

/// Mapper from DOT
impl From<CreatePermissionInput> for Permission {
    fn from(value: CreatePermissionInput) -> Self {
        Self::new(
            value.resource,
            value.action,
            value.description,
            DateTime::default(),
        )
    }
}

/// Membership
#[derive(Debug, Clone)]
pub struct CreateMembershipInput {
    pub user_id: UserId,
    pub tenet_id: TenantId,
    pub roles: HashSet<RoleId>,
}

/// Mapper from DOT
impl From<CreateMembershipInput> for Membership {
    fn from(value: CreateMembershipInput) -> Self {
        Self::new(
            value.user_id,
            value.tenet_id,
            value.roles,
            DateTime::default(),
        )
    }
}

/// TemporaryGrant
#[derive(Debug, Clone)]
pub struct CreateTemporaryGrantInput {
    pub user_id: UserId,
    pub description: Description,
    pub resource: Resource,
    pub action: Action,
    pub expires_at: DateTime,
}

/// Mapper from DOT
impl From<CreateTemporaryGrantInput> for TemporaryGrant {
    fn from(value: CreateTemporaryGrantInput) -> Self {
        Self::new(
            value.user_id,
            value.description,
            value.resource,
            value.action,
            value.expires_at,
            DateTime::default(),
        )
    }
}
