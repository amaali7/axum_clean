use std::collections::HashSet;

use domain::{
    tenant::temporary_grant::TemporaryGrant,
    value_objects::{Action, DateTime, Description, Resource},
    Membership, Name, Permission, RoleId, Tenant, TenantId, UserId,
};

pub struct PrivilegeTenantOutput {
    pub id: TenantId,
    pub name: Name,
    pub description: Description,
    pub create_at: DateTime,
}

/// Mapper from DOT

impl From<Tenant> for PrivilegeTenantOutput {
    fn from(value: Tenant) -> Self {
        Self {
            id: value.id().clone(),
            name: value.name().clone(),
            description: value.description().clone(),
            create_at: value.created_at().clone(),
        }
    }
}

/// permission

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct PrivilegePermissionOutput {
    pub resource: Resource,
    pub action: Action,
    pub description: Description,
    pub create_at: DateTime,
}

/// Mapper from DOT
impl From<Permission> for PrivilegePermissionOutput {
    fn from(value: Permission) -> Self {
        Self {
            resource: value.resource().clone(),
            action: value.action().clone(),
            description: value.description().clone(),
            create_at: DateTime::default().clone(),
        }
    }
}

/// Membership
#[derive(Debug, Clone)]
pub struct PrivilegeMembershipOutput {
    pub user_id: UserId,
    pub tenet_id: TenantId,
    pub roles: HashSet<RoleId>,
    pub create_at: DateTime,
}

/// Mapper from DOT
impl From<Membership> for PrivilegeMembershipOutput {
    fn from(value: Membership) -> Self {
        Self {
            user_id: value.user_id().clone(),
            tenet_id: value.tenet_id().clone(),
            roles: value.roles().clone(),
            create_at: DateTime::default().clone(),
        }
    }
}

/// TemporaryGrant
#[derive(Debug, Clone)]
pub struct PrivilegeTemporaryGrantOutput {
    pub user_id: UserId,
    pub description: Description,
    pub resource: Resource,
    pub action: Action,
    pub expires_at: DateTime,
    pub create_at: DateTime,
}

/// Mapper from DOT
impl From<TemporaryGrant> for PrivilegeTemporaryGrantOutput {
    fn from(value: TemporaryGrant) -> Self {
        Self {
            user_id: value.user_id().clone(),
            description: value.description().clone(),
            resource: value.resource().clone(),
            action: value.action().clone(),
            expires_at: value.expires_at().clone(),
            create_at: DateTime::default().clone(),
        }
    }
}
