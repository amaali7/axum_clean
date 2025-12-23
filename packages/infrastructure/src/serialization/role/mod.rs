pub mod permissions;
pub mod role;

use domain::RoleId;
pub use permissions::SerializedPermission;
pub use role::SerializedRole;
use serde::Serialize;

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
