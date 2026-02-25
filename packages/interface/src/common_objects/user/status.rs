use domain::user::UserStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceUserStatus {
    Active,
    Suspended,
    Inactive,
    Banned,
}

impl From<UserStatus> for InterfaceUserStatus {
    fn from(value: UserStatus) -> Self {
        match value {
            UserStatus::Active => Self::Active,
            UserStatus::Suspended => Self::Suspended,
            UserStatus::Inactive => Self::Inactive,
            UserStatus::Banned => Self::Banned,
        }
    }
}

impl From<InterfaceUserStatus> for UserStatus {
    fn from(value: InterfaceUserStatus) -> Self {
        match value {
            InterfaceUserStatus::Active => Self::Active,
            InterfaceUserStatus::Suspended => Self::Suspended,
            InterfaceUserStatus::Inactive => Self::Inactive,
            InterfaceUserStatus::Banned => Self::Banned,
        }
    }
}

impl Default for InterfaceUserStatus {
    fn default() -> Self {
        InterfaceUserStatus::Inactive
    }
}
