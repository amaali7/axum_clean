pub mod permissions;
pub mod role;

use application::ports::role::RoleQueryResult;
use domain::{Event, Role, RoleId};
pub use permissions::SerializedPermission;
pub use role::SerializedRole;
use serde::{Deserialize, Serialize};
use surrealdb::Response;

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerializedRoleId(String);

impl SerializedRoleId {
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
impl Serialize for SerializedRoleId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// impl<'de> Deserialize<'de> for SerializedRoleId {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let record_id = String::deserialize(deserializer)?;
//         Ok(SerializedRoleId(record_id))
//     }
// }

impl From<RoleId> for SerializedRoleId {
    fn from(value: RoleId) -> Self {
        Self::new(&value.id())
    }
}

impl From<SerializedRoleId> for RoleId {
    fn from(value: SerializedRoleId) -> Self {
        Self::new(&value.id())
    }
}

impl Event for SerializedRole {
    fn get_type(&self) -> &str {
        "ROLE"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SerializedRoleQueryResult {
    Single(SerializedRole),
    Array(Vec<SerializedRole>),
    None,
}

impl SerializedRoleQueryResult {
    pub fn get_array(&self) -> Option<Vec<SerializedRole>> {
        match self {
            SerializedRoleQueryResult::Array(users) => Some(users.to_vec()),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<SerializedRole> {
        match self {
            SerializedRoleQueryResult::Single(user) => Some(user.clone()),
            _ => None,
        }
    }
}

// Extension trait for SurrealDB Response
pub trait SurrealRoleResponseExt {
    async fn into_role_result(self) -> Result<SerializedRoleQueryResult, surrealdb::Error>;
}

impl SurrealRoleResponseExt for Response {
    async fn into_role_result(mut self) -> Result<SerializedRoleQueryResult, surrealdb::Error> {
        // Check if we have any results
        let num_statements = self.num_statements();
        
        if num_statements == 0 {
            return Ok(SerializedRoleQueryResult::None);
        }

        // Try to take the first statement result as a single value
        if let Ok(single) = self.take::<Option<SerializedRole>>(0) {
            if let Some(user) = single {
                return Ok(SerializedRoleQueryResult::Single(user));
            }
        }

        // Try as an array
        if let Ok(array) = self.take::<Vec<SerializedRole>>(0) {
            if array.is_empty() {
                return Ok(SerializedRoleQueryResult::None);
            }
            return Ok(SerializedRoleQueryResult::Array(array));
        }

        Ok(SerializedRoleQueryResult::None)
    }
}

impl TryFrom<RoleQueryResult> for SerializedRoleQueryResult {
    type Error = InfrastructureError;

    fn try_from(value: RoleQueryResult) -> InfrastructureResult<Self> {
        Ok(match value {
            RoleQueryResult::Single(role ) => Self::Single(role.try_into()?),
            RoleQueryResult::Array(roles) => {
                let mut vec_roles: Vec<SerializedRole> = Vec::new();
                for role in roles {
                    vec_roles.push(role.try_into()?);
                }
                Self::Array(vec_roles)
            }
            RoleQueryResult::None => Self::None,
        })
    }
}

impl TryFrom<SerializedRoleQueryResult> for RoleQueryResult {
    type Error = InfrastructureError;

    fn try_from(value: SerializedRoleQueryResult) -> InfrastructureResult<Self> {
        Ok(match value {
            SerializedRoleQueryResult::Single(role) => Self::Single(role.try_into()?),
            SerializedRoleQueryResult::Array(roles) => {
                let mut vec_roles: Vec<Role> = Vec::new();
                for role in roles {
                    vec_roles.push(role.try_into()?);
                }
                Self::Array(vec_roles)
            }
            SerializedRoleQueryResult::None => Self::None,
        })
    }
}

