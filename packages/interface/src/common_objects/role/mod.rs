pub mod permissions;
use domain::RoleId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InterfaceRoleId(String);

impl InterfaceRoleId {
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

impl From<InterfaceRoleId> for RoleId {
    fn from(value: InterfaceRoleId) -> Self {
        Self::new(&value.id())
    }
}

impl From<RoleId> for InterfaceRoleId {
    fn from(value: RoleId) -> Self {
        Self::new(&value.id())
    }
}
