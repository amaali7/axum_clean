use chrono::{DateTime, Utc};

use crate::{Permission, RoleId, UserId};

use super::DomainEvent;

#[derive(Debug, Clone)]
pub enum RoleEvent {
    Created {
        role_id: RoleId,
        created_by: UserId,
        occurred_at: DateTime<Utc>,
    },
    Removed {
        role_id: RoleId,
        removed_by: UserId,
        occurred_at: DateTime<Utc>,
    },
    PermissionAdded {
        role_id: RoleId,
        permissions_added: Vec<Permission>,
        added_by: UserId,
        occurred_at: DateTime<Utc>,
    },
    PermissionRemoved {
        role_id: RoleId,
        permissions_removed: Vec<Permission>,
        removed_by: UserId,
        occurred_at: DateTime<Utc>,
    },
}

impl DomainEvent for RoleEvent {
    fn event_type(&self) -> &'static str {
        match self {
            RoleEvent::Created { .. } => "role.created",
            RoleEvent::Removed { .. } => "role.removed",
            RoleEvent::PermissionAdded { .. } => "role.add-permissions",
            RoleEvent::PermissionRemoved { .. } => "role.remove-permissions",
        }
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        match self {
            RoleEvent::Created { occurred_at, .. } => *occurred_at,
            RoleEvent::Removed { occurred_at, .. } => *occurred_at,
            RoleEvent::PermissionAdded { occurred_at, .. } => *occurred_at,
            RoleEvent::PermissionRemoved { occurred_at, .. } => *occurred_at,
        }
    }
}
