use std::collections::HashSet;

use crate::{DateTime, Event, RoleId, UserId};

use super::TenantId;

#[derive(Debug, Clone)]
pub struct Membership {
    user_id: UserId,
    tenet_id: TenantId,
    roles: HashSet<RoleId>,
    created_at: DateTime,
}

impl Membership {
    pub fn new(
        user_id: UserId,
        tenet_id: TenantId,
        roles: HashSet<RoleId>,
        created_at: DateTime,
    ) -> Self {
        Self {
            user_id,
            tenet_id,
            roles,
            created_at,
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
}

impl Event for Membership {
    fn get_type(&self) -> &str {
        "MEMBERSHIP"
    }
}
