use crate::{value_objects::Diff, RoleId, UserId};

#[derive(Debug, Clone)]
pub enum UserEvent {
    Created {
        user_id: UserId,
        created_by: UserId,
    },
    Removed {
        user_id: UserId,
        removed_by: UserId,
    },
    UpdateUsername {
        user_id: UserId,
        diff: Diff,
    },
    UpdateEmail {
        user_id: UserId,
        diff: Diff,
    },
    UpdateProfile {
        user_id: UserId,
        diff: Diff,
    },
    UpdatePreferences {
        user_id: UserId,
        diff: Diff,
    },
    UpdateRoles {
        user_id: UserId,
        updated_by: UserId,
        roles_ids: Vec<RoleId>,
        diff: Diff,
    },
    UpdatePermissions {
        user_id: UserId,
        updated_by: UserId,
        diff: Diff,
    },
    UpdateStatus {
        user_id: UserId,
        updated_by: UserId,
        diff: Diff,
    },
}

impl UserEvent {
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
}
