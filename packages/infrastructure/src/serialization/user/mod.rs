pub mod preferences;
pub mod profile;
pub mod status;
pub mod user;

use application::ports::user::UserQueryResult;
pub use preferences::SerializedUserPreferences;
pub use profile::SerializedUserProfile;
pub use status::SerializedUserStatus;
use surrealdb::Response;

use domain::{Event, User, UserId};
use serde::{Deserialize, Serialize};
use user::SerializedUser;

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct SerializedUserId(String);

impl SerializedUserId {
    // Create a new SerializedReportId with a UUID
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }

    // Get the inner String for database operations
    pub fn id(&self) -> String {
        self.0.clone()
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Implement Serialize and Deserialize for your type
impl Serialize for SerializedUserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// impl<'de> Deserialize<'de> for SerializedUserId {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let record_id = String::deserialize(deserializer)?;
//         Ok(SerializedUserId(record_id))
//     }
// }

impl From<SerializedUserId> for UserId {
    fn from(value: SerializedUserId) -> Self {
        Self::new(&value.id())
    }
}

impl From<UserId> for SerializedUserId {
    fn from(value: UserId) -> Self {
        Self::new(&value.id())
    }
}
impl Event for SerializedUser {
    fn get_type(&self) -> &str {
        "USER"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SerializedUserQueryResult {
    Single(SerializedUser),
    Array(Vec<SerializedUser>),
    None,
}

impl SerializedUserQueryResult {
    pub fn get_array(&self) -> Option<Vec<SerializedUser>> {
        match self {
            SerializedUserQueryResult::Array(users) => Some(users.to_vec()),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<SerializedUser> {
        match self {
            SerializedUserQueryResult::Single(user) => Some(user.clone()),
            _ => None,
        }
    }
}

// Extension trait for SurrealDB Response
pub trait SurrealUserResponseExt {
    async fn into_user_result(self) -> Result<SerializedUserQueryResult, surrealdb::Error>;
}

impl SurrealUserResponseExt for Response {
    async fn into_user_result(mut self) -> Result<SerializedUserQueryResult, surrealdb::Error> {
        // Check if we have any results
        let num_statements = self.num_statements();
        
        if num_statements == 0 {
            return Ok(SerializedUserQueryResult::None);
        }

        // Try to take the first statement result as a single value
        if let Ok(single) = self.take::<Option<SerializedUser>>(0) {
            if let Some(user) = single {
                return Ok(SerializedUserQueryResult::Single(user));
            }
        }

        // Try as an array
        if let Ok(array) = self.take::<Vec<SerializedUser>>(0) {
            if array.is_empty() {
                return Ok(SerializedUserQueryResult::None);
            }
            return Ok(SerializedUserQueryResult::Array(array));
        }

        Ok(SerializedUserQueryResult::None)
    }
}

impl TryFrom<UserQueryResult> for SerializedUserQueryResult {
    type Error = InfrastructureError;

    fn try_from(value: UserQueryResult) -> InfrastructureResult<Self> {
        Ok(match value {
            UserQueryResult::Single(user) => Self::Single(user.try_into()?),
            UserQueryResult::Array(users) => {
                let mut vec_users: Vec<SerializedUser> = Vec::new();
                for user in users {
                    vec_users.push(user.try_into()?);
                }
                Self::Array(vec_users)
            }
            UserQueryResult::None => Self::None,
        })
    }
}

impl TryFrom<SerializedUserQueryResult> for UserQueryResult {
    type Error = InfrastructureError;

    fn try_from(value: SerializedUserQueryResult) -> InfrastructureResult<Self> {
        Ok(match value {
            SerializedUserQueryResult::Single(user) => Self::Single(user.try_into()?),
            SerializedUserQueryResult::Array(users) => {
                let mut vec_users: Vec<User> = Vec::new();
                for user in users {
                    vec_users.push(user.try_into()?);
                }
                Self::Array(vec_users)
            }
            SerializedUserQueryResult::None => Self::None,
        })
    }
}
