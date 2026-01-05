use domain::events::RoleEvent;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{SerializedPermission, SerializedRoleId, SerializedUserId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedRoleEvent {
    Created {
        role_id: SerializedRoleId,
        created_by: SerializedUserId,
    },
    Removed {
        role_id: SerializedRoleId,
        removed_by: SerializedUserId,
    },
    PermissionAdded {
        role_id: SerializedRoleId,
        permissions_added: Vec<SerializedPermission>,
        added_by: SerializedUserId,
    },
    PermissionRemoved {
        role_id: SerializedRoleId,
        permissions_removed: Vec<SerializedPermission>,
        removed_by: SerializedUserId,
    },
}

impl SerializedRoleEvent {
    pub fn event_type(&self) -> &'static str {
        match self {
            SerializedRoleEvent::Created { .. } => "role.created",
            SerializedRoleEvent::Removed { .. } => "role.removed",
            SerializedRoleEvent::PermissionAdded { .. } => "role.add-permissions",
            SerializedRoleEvent::PermissionRemoved { .. } => "role.remove-permissions",
        }
    }
}

impl TryFrom<RoleEvent> for SerializedRoleEvent {
    fn try_from(value: RoleEvent) -> InfrastructureResult<Self> {
        Ok(match value {
            RoleEvent::Created {
                role_id,
                created_by,
            } => Self::Created {
                role_id: role_id.into(),
                created_by: created_by.into(),
            },
            RoleEvent::Removed {
                role_id,
                removed_by,
            } => Self::Removed {
                role_id: role_id.into(),
                removed_by: removed_by.into(),
            },
            RoleEvent::PermissionAdded {
                role_id,
                permissions_added,
                added_by,
            } => Self::PermissionAdded {
                role_id: role_id.into(),
                permissions_added: permissions_added
                    .into_iter()
                    .map(|value| value.into())
                    .collect(),
                added_by: added_by.into(),
            },
            RoleEvent::PermissionRemoved {
                role_id,
                permissions_removed,
                removed_by,
            } => Self::PermissionRemoved {
                role_id: role_id.into(),
                permissions_removed: permissions_removed
                    .into_iter()
                    .map(|value| value.into())
                    .collect(),
                removed_by: removed_by.into(),
            },
        })
    }

    type Error = InfrastructureError;
}

impl TryFrom<SerializedRoleEvent> for RoleEvent {
    fn try_from(value: SerializedRoleEvent) -> InfrastructureResult<Self> {
        Ok(match value {
            SerializedRoleEvent::Created {
                role_id,
                created_by,
            } => Self::Created {
                role_id: role_id.into(),
                created_by: created_by.into(),
            },
            SerializedRoleEvent::Removed {
                role_id,
                removed_by,
            } => Self::Removed {
                role_id: role_id.into(),
                removed_by: removed_by.into(),
            },
            SerializedRoleEvent::PermissionAdded {
                role_id,
                permissions_added,
                added_by,
            } => Self::PermissionAdded {
                role_id: role_id.into(),
                permissions_added: permissions_added
                    .into_iter()
                    .map(|value| value.into())
                    .collect(),
                added_by: added_by.into(),
            },
            SerializedRoleEvent::PermissionRemoved {
                role_id,
                permissions_removed,
                removed_by,
            } => Self::PermissionRemoved {
                role_id: role_id.into(),
                permissions_removed: permissions_removed
                    .into_iter()
                    .map(|value| value.into())
                    .collect(),
                removed_by: removed_by.into(),
            },
        })
    }

    type Error = InfrastructureError;
}
