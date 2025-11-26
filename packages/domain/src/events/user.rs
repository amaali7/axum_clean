use crate::{value_objects::Diff, RoleId, UserId};

use super::DomainEvent;

#[derive(Debug, Clone)]
pub enum UserEvent {
    Created {
        user_id: UserId,
        created_by: UserId,
        occurred_at: String,
    },
    Removed {
        user_id: UserId,
        removed_by: UserId,
        occurred_at: String,
    },
    UpdateUsername {
        user_id: UserId,
        diff: Diff,
        occurred_at: String,
    },
    UpdateEmail {
        user_id: UserId,
        diff: Diff,
        occurred_at: String,
    },
    UpdateProfile {
        user_id: UserId,
        diff: Diff,
        occurred_at: String,
    },
    UpdatePreferences {
        user_id: UserId,
        diff: Diff,
        occurred_at: String,
    },
    UpdateRoles {
        user_id: UserId,
        updated_by: UserId,
        roles_ids: Vec<RoleId>,
        diff: Diff,
        occurred_at: String,
    },
    UpdatePermissions {
        user_id: UserId,
        updated_by: UserId,
        diff: Diff,
        occurred_at: String,
    },
    UpdateStatus {
        user_id: UserId,
        updated_by: UserId,
        diff: Diff,
        occurred_at: String,
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

    fn occurred_at(&self) -> String {
        match self {
            UserEvent::Created { occurred_at, .. } => occurred_at.clone(),
            UserEvent::Removed { occurred_at, .. } => occurred_at.clone(),
            UserEvent::UpdateUsername { occurred_at, .. } => occurred_at.clone(),
            UserEvent::UpdateEmail { occurred_at, .. } => occurred_at.clone(),
            UserEvent::UpdateProfile { occurred_at, .. } => occurred_at.clone(),
            UserEvent::UpdatePreferences { occurred_at, .. } => occurred_at.clone(),
            UserEvent::UpdateRoles { occurred_at, .. } => occurred_at.clone(),
            UserEvent::UpdatePermissions { occurred_at, .. } => occurred_at.clone(),
            UserEvent::UpdateStatus { occurred_at, .. } => occurred_at.clone(),
        }
    }
}
