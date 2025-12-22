use domain::events::UserEvent;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::date_time::SerializedDateTime, SerializedDiff, SerializedRoleId,
        SerializedUserId,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedUserEvent {
    Created {
        user_id: SerializedUserId,
        created_by: SerializedUserId,
        occurred_at: SerializedDateTime,
    },
    Removed {
        user_id: SerializedUserId,
        removed_by: SerializedUserId,
        occurred_at: SerializedDateTime,
    },
    UpdateUsername {
        user_id: SerializedUserId,
        diff: SerializedDiff,
        occurred_at: SerializedDateTime,
    },
    UpdateEmail {
        user_id: SerializedUserId,
        diff: SerializedDiff,
        occurred_at: SerializedDateTime,
    },
    UpdateProfile {
        user_id: SerializedUserId,
        diff: SerializedDiff,
        occurred_at: SerializedDateTime,
    },
    UpdatePreferences {
        user_id: SerializedUserId,
        diff: SerializedDiff,
        occurred_at: SerializedDateTime,
    },
    UpdateRoles {
        user_id: SerializedUserId,
        updated_by: SerializedUserId,
        roles_ids: Vec<SerializedRoleId>,
        diff: SerializedDiff,
        occurred_at: SerializedDateTime,
    },
    UpdatePermissions {
        user_id: SerializedUserId,
        updated_by: SerializedUserId,
        diff: SerializedDiff,
        occurred_at: SerializedDateTime,
    },
    UpdateStatus {
        user_id: SerializedUserId,
        updated_by: SerializedUserId,
        diff: SerializedDiff,
        occurred_at: SerializedDateTime,
    },
}

impl SerializedUserEvent {
    pub fn event_type(&self) -> &'static str {
        match self {
            SerializedUserEvent::Created { .. } => "user.created",
            SerializedUserEvent::Removed { .. } => "user.removed",
            SerializedUserEvent::UpdateUsername { .. } => "user.update.username",
            SerializedUserEvent::UpdateEmail { .. } => "user.update.email",
            SerializedUserEvent::UpdateProfile { .. } => "user.update.profile",
            SerializedUserEvent::UpdatePreferences { .. } => "user.update.preferences",
            SerializedUserEvent::UpdateRoles { .. } => "user.update.roles",
            SerializedUserEvent::UpdatePermissions { .. } => "user.update.permissions",
            SerializedUserEvent::UpdateStatus { .. } => "user.update.status",
        }
    }

    pub fn occurred_at(&self) -> SerializedDateTime {
        match self {
            SerializedUserEvent::Created { occurred_at, .. } => occurred_at.clone(),
            SerializedUserEvent::Removed { occurred_at, .. } => occurred_at.clone(),
            SerializedUserEvent::UpdateUsername { occurred_at, .. } => occurred_at.clone(),
            SerializedUserEvent::UpdateEmail { occurred_at, .. } => occurred_at.clone(),
            SerializedUserEvent::UpdateProfile { occurred_at, .. } => occurred_at.clone(),
            SerializedUserEvent::UpdatePreferences { occurred_at, .. } => occurred_at.clone(),
            SerializedUserEvent::UpdateRoles { occurred_at, .. } => occurred_at.clone(),
            SerializedUserEvent::UpdatePermissions { occurred_at, .. } => occurred_at.clone(),
            SerializedUserEvent::UpdateStatus { occurred_at, .. } => occurred_at.clone(),
        }
    }
}

impl TryFrom<UserEvent> for SerializedUserEvent {
    fn try_from(value: UserEvent) -> InfrastructureResult<Self> {
        Ok(match value {
            UserEvent::Created {
                user_id,
                created_by,
                occurred_at,
            } => Self::Created {
                user_id: user_id.into(),
                created_by: created_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
            UserEvent::Removed {
                user_id,
                removed_by,
                occurred_at,
            } => Self::Removed {
                user_id: user_id.into(),
                removed_by: removed_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
            UserEvent::UpdateUsername {
                user_id,
                diff,
                occurred_at,
            } => Self::UpdateUsername {
                user_id: user_id.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            UserEvent::UpdateEmail {
                user_id,
                diff,
                occurred_at,
            } => Self::UpdateEmail {
                user_id: user_id.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            UserEvent::UpdateProfile {
                user_id,
                diff,
                occurred_at,
            } => Self::UpdateProfile {
                user_id: user_id.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            UserEvent::UpdatePreferences {
                user_id,
                diff,
                occurred_at,
            } => Self::UpdatePreferences {
                user_id: user_id.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            UserEvent::UpdateRoles {
                user_id,
                updated_by,
                roles_ids,
                diff,
                occurred_at,
            } => Self::UpdateRoles {
                user_id: user_id.into(),
                updated_by: updated_by.into(),
                roles_ids: roles_ids.into_iter().map(|id| id.into()).collect(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            UserEvent::UpdatePermissions {
                user_id,
                updated_by,
                diff,
                occurred_at,
            } => Self::UpdatePermissions {
                user_id: user_id.into(),
                updated_by: updated_by.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            UserEvent::UpdateStatus {
                user_id,
                updated_by,
                diff,
                occurred_at,
            } => Self::UpdateStatus {
                user_id: user_id.into(),
                updated_by: updated_by.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
        })
    }

    type Error = InfrastructureError;
}

impl TryFrom<SerializedUserEvent> for UserEvent {
    fn try_from(value: SerializedUserEvent) -> InfrastructureResult<Self> {
        Ok(match value {
            SerializedUserEvent::Created {
                user_id,
                created_by,
                occurred_at,
            } => Self::Created {
                user_id: user_id.into(),
                created_by: created_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedUserEvent::Removed {
                user_id,
                removed_by,
                occurred_at,
            } => Self::Removed {
                user_id: user_id.into(),
                removed_by: removed_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedUserEvent::UpdateUsername {
                user_id,
                diff,
                occurred_at,
            } => Self::UpdateUsername {
                user_id: user_id.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedUserEvent::UpdateEmail {
                user_id,
                diff,
                occurred_at,
            } => Self::UpdateEmail {
                user_id: user_id.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedUserEvent::UpdateProfile {
                user_id,
                diff,
                occurred_at,
            } => Self::UpdateProfile {
                user_id: user_id.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedUserEvent::UpdatePreferences {
                user_id,
                diff,
                occurred_at,
            } => Self::UpdatePreferences {
                user_id: user_id.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedUserEvent::UpdateRoles {
                user_id,
                updated_by,
                roles_ids,
                diff,
                occurred_at,
            } => Self::UpdateRoles {
                user_id: user_id.into(),
                updated_by: updated_by.into(),
                roles_ids: roles_ids.into_iter().map(|id| id.into()).collect(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedUserEvent::UpdatePermissions {
                user_id,
                updated_by,
                diff,
                occurred_at,
            } => Self::UpdatePermissions {
                user_id: user_id.into(),
                updated_by: updated_by.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedUserEvent::UpdateStatus {
                user_id,
                updated_by,
                diff,
                occurred_at,
            } => Self::UpdateStatus {
                user_id: user_id.into(),
                updated_by: updated_by.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
        })
    }

    type Error = InfrastructureError;
}
