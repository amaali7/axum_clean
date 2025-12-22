use domain::events::RoleEvent;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::date_time::SerializedDateTime, SerializedPermission, SerializedRoleId,
        SerializedUserId,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedRoleEvent {
    Created {
        role_id: SerializedRoleId,
        created_by: SerializedUserId,
        occurred_at: SerializedDateTime,
    },
    Removed {
        role_id: SerializedRoleId,
        removed_by: SerializedUserId,
        occurred_at: SerializedDateTime,
    },
    PermissionAdded {
        role_id: SerializedRoleId,
        permissions_added: Vec<SerializedPermission>,
        added_by: SerializedUserId,
        occurred_at: SerializedDateTime,
    },
    PermissionRemoved {
        role_id: SerializedRoleId,
        permissions_removed: Vec<SerializedPermission>,
        removed_by: SerializedUserId,
        occurred_at: SerializedDateTime,
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

    pub fn occurred_at(&self) -> SerializedDateTime {
        match self {
            SerializedRoleEvent::Created { occurred_at, .. } => occurred_at.clone(),
            SerializedRoleEvent::Removed { occurred_at, .. } => occurred_at.clone(),
            SerializedRoleEvent::PermissionAdded { occurred_at, .. } => occurred_at.clone(),
            SerializedRoleEvent::PermissionRemoved { occurred_at, .. } => occurred_at.clone(),
        }
    }
}

impl TryFrom<RoleEvent> for SerializedRoleEvent {
    fn try_from(value: RoleEvent) -> InfrastructureResult<Self> {
        Ok(match value {
            RoleEvent::Created {
                role_id,
                created_by,
                occurred_at,
            } => Self::Created {
                role_id: role_id.into(),
                created_by: created_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
            RoleEvent::Removed {
                role_id,
                removed_by,
                occurred_at,
            } => Self::Removed {
                role_id: role_id.into(),
                removed_by: removed_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
            RoleEvent::PermissionAdded {
                role_id,
                permissions_added,
                added_by,
                occurred_at,
            } => Self::PermissionAdded {
                role_id: role_id.into(),
                permissions_added: permissions_added
                    .into_iter()
                    .map(|value| value.into())
                    .collect(),
                added_by: added_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
            RoleEvent::PermissionRemoved {
                role_id,
                permissions_removed,
                removed_by,
                occurred_at,
            } => Self::PermissionRemoved {
                role_id: role_id.into(),
                permissions_removed: permissions_removed
                    .into_iter()
                    .map(|value| value.into())
                    .collect(),
                removed_by: removed_by.into(),
                occurred_at: occurred_at.try_into()?,
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
                occurred_at,
            } => Self::Created {
                role_id: role_id.into(),
                created_by: created_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedRoleEvent::Removed {
                role_id,
                removed_by,
                occurred_at,
            } => Self::Removed {
                role_id: role_id.into(),
                removed_by: removed_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedRoleEvent::PermissionAdded {
                role_id,
                permissions_added,
                added_by,
                occurred_at,
            } => Self::PermissionAdded {
                role_id: role_id.into(),
                permissions_added: permissions_added
                    .into_iter()
                    .map(|value| value.into())
                    .collect(),
                added_by: added_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedRoleEvent::PermissionRemoved {
                role_id,
                permissions_removed,
                removed_by,
                occurred_at,
            } => Self::PermissionRemoved {
                role_id: role_id.into(),
                permissions_removed: permissions_removed
                    .into_iter()
                    .map(|value| value.into())
                    .collect(),
                removed_by: removed_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
        })
    }

    type Error = InfrastructureError;
}
