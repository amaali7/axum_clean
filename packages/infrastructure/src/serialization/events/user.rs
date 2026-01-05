use domain::events::UserEvent;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{SerializedDiff, SerializedRoleId, SerializedUserId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedUserEvent {
    Created {
        user_id: SerializedUserId,
        created_by: SerializedUserId,
    },
    Removed {
        user_id: SerializedUserId,
        removed_by: SerializedUserId,
    },
    UpdateUsername {
        user_id: SerializedUserId,
        diff: SerializedDiff,
    },
    UpdateEmail {
        user_id: SerializedUserId,
        diff: SerializedDiff,
    },
    UpdateProfile {
        user_id: SerializedUserId,
        diff: SerializedDiff,
    },
    UpdatePreferences {
        user_id: SerializedUserId,
        diff: SerializedDiff,
    },
    UpdateRoles {
        user_id: SerializedUserId,
        updated_by: SerializedUserId,
        roles_ids: Vec<SerializedRoleId>,
        diff: SerializedDiff,
    },
    UpdatePermissions {
        user_id: SerializedUserId,
        updated_by: SerializedUserId,
        diff: SerializedDiff,
    },
    UpdateStatus {
        user_id: SerializedUserId,
        updated_by: SerializedUserId,
        diff: SerializedDiff,
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
}

impl TryFrom<UserEvent> for SerializedUserEvent {
    fn try_from(value: UserEvent) -> InfrastructureResult<Self> {
        Ok(match value {
            UserEvent::Created {
                user_id,
                created_by,
            } => Self::Created {
                user_id: user_id.into(),
                created_by: created_by.into(),
            },
            UserEvent::Removed {
                user_id,
                removed_by,
            } => Self::Removed {
                user_id: user_id.into(),
                removed_by: removed_by.into(),
            },
            UserEvent::UpdateUsername { user_id, diff } => Self::UpdateUsername {
                user_id: user_id.into(),
                diff: diff.into(),
            },
            UserEvent::UpdateEmail { user_id, diff } => Self::UpdateEmail {
                user_id: user_id.into(),
                diff: diff.into(),
            },
            UserEvent::UpdateProfile { user_id, diff } => Self::UpdateProfile {
                user_id: user_id.into(),
                diff: diff.into(),
            },
            UserEvent::UpdatePreferences { user_id, diff } => Self::UpdatePreferences {
                user_id: user_id.into(),
                diff: diff.into(),
            },
            UserEvent::UpdateRoles {
                user_id,
                updated_by,
                roles_ids,
                diff,
            } => Self::UpdateRoles {
                user_id: user_id.into(),
                updated_by: updated_by.into(),
                roles_ids: roles_ids.into_iter().map(|id| id.into()).collect(),
                diff: diff.into(),
            },
            UserEvent::UpdatePermissions {
                user_id,
                updated_by,
                diff,
            } => Self::UpdatePermissions {
                user_id: user_id.into(),
                updated_by: updated_by.into(),
                diff: diff.into(),
            },
            UserEvent::UpdateStatus {
                user_id,
                updated_by,
                diff,
            } => Self::UpdateStatus {
                user_id: user_id.into(),
                updated_by: updated_by.into(),
                diff: diff.into(),
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
            } => Self::Created {
                user_id: user_id.into(),
                created_by: created_by.into(),
            },
            SerializedUserEvent::Removed {
                user_id,
                removed_by,
            } => Self::Removed {
                user_id: user_id.into(),
                removed_by: removed_by.into(),
            },
            SerializedUserEvent::UpdateUsername { user_id, diff } => Self::UpdateUsername {
                user_id: user_id.into(),
                diff: diff.into(),
            },
            SerializedUserEvent::UpdateEmail { user_id, diff } => Self::UpdateEmail {
                user_id: user_id.into(),
                diff: diff.into(),
            },
            SerializedUserEvent::UpdateProfile { user_id, diff } => Self::UpdateProfile {
                user_id: user_id.into(),
                diff: diff.into(),
            },
            SerializedUserEvent::UpdatePreferences { user_id, diff } => Self::UpdatePreferences {
                user_id: user_id.into(),
                diff: diff.into(),
            },
            SerializedUserEvent::UpdateRoles {
                user_id,
                updated_by,
                roles_ids,
                diff,
            } => Self::UpdateRoles {
                user_id: user_id.into(),
                updated_by: updated_by.into(),
                roles_ids: roles_ids.into_iter().map(|id| id.into()).collect(),
                diff: diff.into(),
            },
            SerializedUserEvent::UpdatePermissions {
                user_id,
                updated_by,
                diff,
            } => Self::UpdatePermissions {
                user_id: user_id.into(),
                updated_by: updated_by.into(),
                diff: diff.into(),
            },
            SerializedUserEvent::UpdateStatus {
                user_id,
                updated_by,
                diff,
            } => Self::UpdateStatus {
                user_id: user_id.into(),
                updated_by: updated_by.into(),
                diff: diff.into(),
            },
        })
    }

    type Error = InfrastructureError;
}
