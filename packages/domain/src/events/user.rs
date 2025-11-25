use chrono::{DateTime, Utc};

use crate::{value_objects::Diff, RoleId, UserId};

use super::DomainEvent;

#[derive(Debug, Clone)]
pub enum UserEvent {
    Created {
        user_id: UserId,
        created_by: UserId,
        occurred_at: DateTime<Utc>,
    },
    Removed {
        user_id: UserId,
        removed_by: UserId,
        occurred_at: DateTime<Utc>,
    },
    UpdateUsername {
        user_id: UserId,
        diff: Diff,
        occurred_at: DateTime<Utc>,
    },
    UpdateEmail {
        user_id: UserId,
        diff: Diff,
        occurred_at: DateTime<Utc>,
    },
    UpdateProfile {
        user_id: UserId,
        diff: Diff,
        occurred_at: DateTime<Utc>,
    },
    UpdatePreferences {
        user_id: UserId,
        diff: Diff,
        occurred_at: DateTime<Utc>,
    },
    UpdateRoles {
        user_id: UserId,
        updated_by: UserId,
        roles_ids: Vec<RoleId>,
        diff: Diff,
        occurred_at: DateTime<Utc>,
    },
    UpdatePermissions {
        user_id: UserId,
        updated_by: UserId,
        diff: Diff,
        occurred_at: DateTime<Utc>,
    },
    UpdateStatus {
        user_id: UserId,
        updated_by: UserId,
        diff: Diff,
        occurred_at: DateTime<Utc>,
    },
}

impl DomainEvent for UserEvent {
    fn event_type(&self) -> &'static str {
        match self {
            UserEvent::Created { .. } => "user.created",
            UserEvent::Removed { .. } => "user.removed",
            UserEvent::UpdateUsername { .. } => "user.update.username",
            UserEvent::UpdateEmail { .. } => "user.update.email",
            UserEvent::UpdateProfile { .. } => "user.update.profile",
            UserEvent::UpdatePreferences { .. } => "user.update.preferences",
            UserEvent::UpdateRoles { .. } => "user.update.roles",
            UserEvent::UpdatePermissions { .. } => "user.update.permissions",
            UserEvent::UpdateStatus { .. } => "user.update.status",
        }
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        match self {
            UserEvent::Created { occurred_at, .. } => *occurred_at,
            UserEvent::Removed { occurred_at, .. } => *occurred_at,
            UserEvent::UpdateUsername { occurred_at, .. } => *occurred_at,
            UserEvent::UpdateEmail { occurred_at, .. } => *occurred_at,
            UserEvent::UpdateProfile { occurred_at, .. } => *occurred_at,
            UserEvent::UpdatePreferences { occurred_at, .. } => *occurred_at,
            UserEvent::UpdateRoles { occurred_at, .. } => *occurred_at,
            UserEvent::UpdatePermissions { occurred_at, .. } => *occurred_at,
            UserEvent::UpdateStatus { occurred_at, .. } => *occurred_at,
        }
    }
}
