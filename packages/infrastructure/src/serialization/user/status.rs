use domain::user::UserStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfrastructureUserStatus {
    Active,
    Suspended,
    Inactive,
    Banned,
}

impl From<UserStatus> for InfrastructureUserStatus {
    fn from(value: UserStatus) -> Self {
        match value {
            UserStatus::Active => Self::Active,
            UserStatus::Suspended => Self::Suspended,
            UserStatus::Inactive => Self::Inactive,
            UserStatus::Banned => Self::Banned,
        }
    }
}

impl From<InfrastructureUserStatus> for UserStatus {
    fn from(value: InfrastructureUserStatus) -> Self {
        match value {
            InfrastructureUserStatus::Active => Self::Active,
            InfrastructureUserStatus::Suspended => Self::Suspended,
            InfrastructureUserStatus::Inactive => Self::Inactive,
            InfrastructureUserStatus::Banned => Self::Banned,
        }
    }
}

impl Default for InfrastructureUserStatus {
    fn default() -> Self {
        InfrastructureUserStatus::Inactive
    }
}
