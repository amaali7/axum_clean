use domain::user::UserStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SerializedUserStatus {
    Active,
    Suspended,
    Inactive,
    Banned,
}

impl From<UserStatus> for SerializedUserStatus {
    fn from(value: UserStatus) -> Self {
        match value {
            UserStatus::Active => Self::Active,
            UserStatus::Suspended => Self::Suspended,
            UserStatus::Inactive => Self::Inactive,
            UserStatus::Banned => Self::Banned,
        }
    }
}

impl From<SerializedUserStatus> for UserStatus {
    fn from(value: SerializedUserStatus) -> Self {
        match value {
            SerializedUserStatus::Active => Self::Active,
            SerializedUserStatus::Suspended => Self::Suspended,
            SerializedUserStatus::Inactive => Self::Inactive,
            SerializedUserStatus::Banned => Self::Banned,
        }
    }
}

impl Default for SerializedUserStatus {
    fn default() -> Self {
        SerializedUserStatus::Inactive
    }
}
