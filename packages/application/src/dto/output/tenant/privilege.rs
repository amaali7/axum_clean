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
            id: value.id(),
            name: value.name(),
            description: value.description(),
            create_at: value.created_at(),
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
            resource: value.resource(),
            action: value.action(),
            description: value.description(),
            create_at: DateTime::default(),
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
            user_id: value.user_id(),
            tenet_id: value.tenet_id(),
            roles: value.roles(),
            create_at: DateTime::default(),
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
            user_id: value.user_id(),
            description: value.description(),
            resource: value.resource(),
            action: value.action(),
            expires_at: value.expires_at(),
            create_at: DateTime::default(),
        }
    }
}
