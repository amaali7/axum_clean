use std::collections::HashSet;

use crate::{DateTime, Event, RoleId, UserId};

use super::TenantId;

#[derive(Debug, Clone)]
pub struct Membership {
    user_id: UserId,
    tenet_id: TenantId,
    roles: HashSet<RoleId>,
    created_at: DateTime,
    version: u64,
}

#[derive(Debug, Clone)]
pub struct MembershipParts {
    pub user_id: UserId,
    pub tenet_id: TenantId,
    pub roles: HashSet<RoleId>,
    pub created_at: DateTime,
    pub version: u64,
}

impl Membership {
    pub fn new(
        user_id: UserId,
        tenet_id: TenantId,
        roles: HashSet<RoleId>,
        created_at: DateTime,
        version: u64,
    ) -> Self {
        Self {
            user_id,
            tenet_id,
            roles,
            created_at,
            version,
        }
    }

    pub fn into_parts(self) -> MembershipParts {
        let Self {
            user_id,
            tenet_id,
            roles,
            created_at,
            version,
        } = self;
        MembershipParts {
            user_id,
            tenet_id,
            roles,
            created_at,
            version,
        }
    }

    pub fn has_role(&self, role_id: &RoleId) -> bool {
        self.roles.iter().any(|p| p == role_id)
    }

    pub fn tenet_id(&self) -> &TenantId {
        &self.tenet_id
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
    pub fn roles(&self) -> &HashSet<RoleId> {
        &self.roles
    }

    pub fn created_at(&self) -> &DateTime {
        &self.created_at
    }

    pub fn version(&self) -> &u64 {
        &self.version
    }
}

impl Event for Membership {
    fn get_type(&self) -> &str {
        "MEMBERSHIP"
    }
}
