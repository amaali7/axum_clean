use crate::{Permission, RoleId, UserId};

#[derive(Debug, Clone)]
pub enum RoleEvent {
    Created {
        role_id: RoleId,
        created_by: UserId,
    },
    Removed {
        role_id: RoleId,
        removed_by: UserId,
    },
    PermissionAdded {
        role_id: RoleId,
        permissions_added: Vec<Permission>,
        added_by: UserId,
    },
    PermissionRemoved {
        role_id: RoleId,
        permissions_removed: Vec<Permission>,
        removed_by: UserId,
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
}
