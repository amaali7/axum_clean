pub mod preferences;
pub mod profile;
pub mod status;
pub mod user;

use application::ports::user::UserQueryResult;
pub use preferences::InfrastructureUserPreferences;
pub use profile::InfrastructureUserProfile;
pub use status::InfrastructureUserStatus;
use surrealdb::Response;

use domain::{Event, User, UserId};
use serde::{Deserialize, Serialize};
use user::InfrastructureUser;

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct InfrastructureUserId(String);

impl InfrastructureUserId {
    // Create a new InfrastructureReportId with a UUID
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
impl Serialize for InfrastructureUserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// impl<'de> Deserialize<'de> for InfrastructureUserId {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let record_id = String::deserialize(deserializer)?;
//         Ok(InfrastructureUserId(record_id))
//     }
// }

impl From<InfrastructureUserId> for UserId {
    fn from(value: InfrastructureUserId) -> Self {
        Self::new(&value.id())
    }
}

impl From<UserId> for InfrastructureUserId {
    fn from(value: UserId) -> Self {
        Self::new(&value.id())
    }
}
impl Event for InfrastructureUser {
    fn get_type(&self) -> &str {
        "USER"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InfrastructureUserQueryResult {
    Single(InfrastructureUser),
    Array(Vec<InfrastructureUser>),
    None,
}

impl InfrastructureUserQueryResult {
    pub fn get_array(&self) -> Option<Vec<InfrastructureUser>> {
        match self {
            InfrastructureUserQueryResult::Array(users) => Some(users.to_vec()),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<InfrastructureUser> {
        match self {
            InfrastructureUserQueryResult::Single(user) => Some(user.clone()),
            _ => None,
        }
    }
}

// Extension trait for SurrealDB Response
pub trait SurrealUserResponseExt {
    async fn into_user_result(self) -> Result<InfrastructureUserQueryResult, surrealdb::Error>;
}

impl SurrealUserResponseExt for Response {
    async fn into_user_result(mut self) -> Result<InfrastructureUserQueryResult, surrealdb::Error> {
        // Check if we have any results
        let num_statements = self.num_statements();
        
        if num_statements == 0 {
            return Ok(InfrastructureUserQueryResult::None);
        }

        // Try to take the first statement result as a single value
        if let Ok(single) = self.take::<Option<InfrastructureUser>>(0) {
            if let Some(user) = single {
                return Ok(InfrastructureUserQueryResult::Single(user));
            }
        }

        // Try as an array
        if let Ok(array) = self.take::<Vec<InfrastructureUser>>(0) {
            if array.is_empty() {
                return Ok(InfrastructureUserQueryResult::None);
            }
            return Ok(InfrastructureUserQueryResult::Array(array));
        }

        Ok(InfrastructureUserQueryResult::None)
    }
}

impl TryFrom<UserQueryResult> for InfrastructureUserQueryResult {
    type Error = InfrastructureError;

    fn try_from(value: UserQueryResult) -> InfrastructureResult<Self> {
        Ok(match value {
            UserQueryResult::Single(user) => Self::Single(user.try_into()?),
            UserQueryResult::Array(users) => {
                let mut vec_users: Vec<InfrastructureUser> = Vec::new();
                for user in users {
                    vec_users.push(user.try_into()?);
                }
                Self::Array(vec_users)
            }
            UserQueryResult::None => Self::None,
        })
    }
}

impl TryFrom<InfrastructureUserQueryResult> for UserQueryResult {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureUserQueryResult) -> InfrastructureResult<Self> {
        Ok(match value {
            InfrastructureUserQueryResult::Single(user) => Self::Single(user.try_into()?),
            InfrastructureUserQueryResult::Array(users) => {
                let mut vec_users: Vec<User> = Vec::new();
                for user in users {
                    vec_users.push(user.try_into()?);
                }
                Self::Array(vec_users)
            }
            InfrastructureUserQueryResult::None => Self::None,
        })
    }
}
