use crate::{Permission, RoleId, UserId};

#[derive(Debug, Clone)]
pub enum RoleEvent {
    Created {
        role_id: RoleId,
        created_by: UserId,
        occurred_at: String,
    },
    Removed {
        role_id: RoleId,
        removed_by: UserId,
        occurred_at: String,
    },
    PermissionAdded {
        role_id: RoleId,
        permissions_added: Vec<Permission>,
        added_by: UserId,
        occurred_at: String,
    },
    PermissionRemoved {
        role_id: RoleId,
        permissions_removed: Vec<Permission>,
        removed_by: UserId,
        occurred_at: String,
    },
}

impl RoleEvent {
    fn event_type(&self) -> &'static str {
        match self {
            RoleEvent::Created { .. } => "role.created",
            RoleEvent::Removed { .. } => "role.removed",
            RoleEvent::PermissionAdded { .. } => "role.add-permissions",
            RoleEvent::PermissionRemoved { .. } => "role.remove-permissions",
        }
    }

    fn occurred_at(&self) -> String {
        match self {
            RoleEvent::Created { occurred_at, .. } => occurred_at.clone(),
            RoleEvent::Removed { occurred_at, .. } => occurred_at.clone(),
            RoleEvent::PermissionAdded { occurred_at, .. } => occurred_at.clone(),
            RoleEvent::PermissionRemoved { occurred_at, .. } => occurred_at.clone(),
        }
    }
}
