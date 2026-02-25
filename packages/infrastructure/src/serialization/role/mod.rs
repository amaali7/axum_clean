pub mod permissions;
pub mod role;

use application::ports::role::RoleQueryResult;
use domain::{Event, Role, RoleId};
pub use permissions::InfrastructurePermission;
pub use role::InfrastructureRole;
use serde::{Deserialize, Serialize};
use surrealdb::Response;

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfrastructureRoleId(String);

impl InfrastructureRoleId {
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
impl Serialize for InfrastructureRoleId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// impl<'de> Deserialize<'de> for InfrastructureRoleId {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let record_id = String::deserialize(deserializer)?;
//         Ok(InfrastructureRoleId(record_id))
//     }
// }

impl From<RoleId> for InfrastructureRoleId {
    fn from(value: RoleId) -> Self {
        Self::new(&value.id())
    }
}

impl From<InfrastructureRoleId> for RoleId {
    fn from(value: InfrastructureRoleId) -> Self {
        Self::new(&value.id())
    }
}

impl Event for InfrastructureRole {
    fn get_type(&self) -> &str {
        "ROLE"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InfrastructureRoleQueryResult {
    Single(InfrastructureRole),
    Array(Vec<InfrastructureRole>),
    None,
}

impl InfrastructureRoleQueryResult {
    pub fn get_array(&self) -> Option<Vec<InfrastructureRole>> {
        match self {
            InfrastructureRoleQueryResult::Array(users) => Some(users.to_vec()),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<InfrastructureRole> {
        match self {
            InfrastructureRoleQueryResult::Single(user) => Some(user.clone()),
            _ => None,
        }
    }
}

// Extension trait for SurrealDB Response
pub trait SurrealRoleResponseExt {
    async fn into_role_result(self) -> Result<InfrastructureRoleQueryResult, surrealdb::Error>;
}

impl SurrealRoleResponseExt for Response {
    async fn into_role_result(mut self) -> Result<InfrastructureRoleQueryResult, surrealdb::Error> {
        // Check if we have any results
        let num_statements = self.num_statements();
        
        if num_statements == 0 {
            return Ok(InfrastructureRoleQueryResult::None);
        }

        // Try to take the first statement result as a single value
        if let Ok(single) = self.take::<Option<InfrastructureRole>>(0) {
            if let Some(user) = single {
                return Ok(InfrastructureRoleQueryResult::Single(user));
            }
        }

        // Try as an array
        if let Ok(array) = self.take::<Vec<InfrastructureRole>>(0) {
            if array.is_empty() {
                return Ok(InfrastructureRoleQueryResult::None);
            }
            return Ok(InfrastructureRoleQueryResult::Array(array));
        }

        Ok(InfrastructureRoleQueryResult::None)
    }
}

impl TryFrom<RoleQueryResult> for InfrastructureRoleQueryResult {
    type Error = InfrastructureError;

    fn try_from(value: RoleQueryResult) -> InfrastructureResult<Self> {
        Ok(match value {
            RoleQueryResult::Single(role ) => Self::Single(role.try_into()?),
            RoleQueryResult::Array(roles) => {
                let mut vec_roles: Vec<InfrastructureRole> = Vec::new();
                for role in roles {
                    vec_roles.push(role.try_into()?);
                }
                Self::Array(vec_roles)
            }
            RoleQueryResult::None => Self::None,
        })
    }
}

impl TryFrom<InfrastructureRoleQueryResult> for RoleQueryResult {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureRoleQueryResult) -> InfrastructureResult<Self> {
        Ok(match value {
            InfrastructureRoleQueryResult::Single(role) => Self::Single(role.try_into()?),
            InfrastructureRoleQueryResult::Array(roles) => {
                let mut vec_roles: Vec<Role> = Vec::new();
                for role in roles {
                    vec_roles.push(role.try_into()?);
                }
                Self::Array(vec_roles)
            }
            InfrastructureRoleQueryResult::None => Self::None,
        })
    }
}

