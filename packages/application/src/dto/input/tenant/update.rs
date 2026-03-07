use std::collections::HashSet;

use domain::{
    tenant::temporary_grant::TemporaryGrant,
    value_objects::{Action, DateTime, Description, Resource},
    Membership, Name, Permission, RoleId, Tenant, TenantId, UserId,
};

pub struct UpdateTenantInput {
    pub id: Option<TenantId>,
    pub name: Option<Name>,
    pub description: Option<Description>,
}

/// Mapper from DOT

impl From<UpdateTenantInput> for Tenant {
    fn from(value: UpdateTenantInput) -> Self {
        Self::new(
            value.id.unwrap_or_default(),
            value.name.unwrap_or_default(),
            value.description.unwrap_or_default(),
            DateTime::default(),
        )
    }
}

/// permission

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct UpdatePermissionInput {
    pub resource: Option<Resource>,
    pub action: Option<Action>,
    pub description: Option<Description>,
}

/// Mapper from DOT
impl From<UpdatePermissionInput> for Permission {
    fn from(value: UpdatePermissionInput) -> Self {
        Self::new(
            value.resource.unwrap_or_default(),
            value.action.unwrap_or_default(),
            value.description.unwrap_or_default(),
            DateTime::default(),
        )
    }
}

/// Membership
#[derive(Debug, Clone)]
pub struct UpdateMembershipInput {
    pub user_id: Option<UserId>,
    pub tenet_id: Option<TenantId>,
    pub roles: HashSet<RoleId>,
}

/// Mapper from DOT
impl From<UpdateMembershipInput> for Membership {
    fn from(value: UpdateMembershipInput) -> Self {
        Self::new(
            value.user_id.unwrap_or_default(),
            value.tenet_id.unwrap_or_default(),
            value.roles,
            DateTime::default(),
        )
    }
}

/// TemporaryGrant
#[derive(Debug, Clone)]
pub struct UpdateTemporaryGrantInput {
    pub user_id: Option<UserId>,
    pub description: Option<Description>,
    pub resource: Option<Resource>,
    pub action: Option<Action>,
    pub expires_at: Option<DateTime>,
}

/// Mapper from DOT
impl From<UpdateTemporaryGrantInput> for TemporaryGrant {
    fn from(value: UpdateTemporaryGrantInput) -> Self {
        Self::new(
            value.user_id.unwrap_or_default(),
            value.description.unwrap_or_default(),
            value.resource.unwrap_or_default(),
            value.action.unwrap_or_default(),
            value.expires_at.unwrap_or_default(),
            DateTime::default(),
        )
    }
}
