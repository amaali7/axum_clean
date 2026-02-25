pub mod status;
use domain::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InterfaceUserId(String);

impl InterfaceUserId {
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

impl From<InterfaceUserId> for UserId {
    fn from(value: InterfaceUserId) -> Self {
        Self::new(&value.id())
    }
}

impl From<UserId> for InterfaceUserId {
    fn from(value: UserId) -> Self {
        Self::new(&value.id())
    }
}
