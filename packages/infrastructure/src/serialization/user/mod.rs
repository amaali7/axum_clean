pub mod preferences;
pub mod profile;
pub mod status;
pub mod user;

pub use preferences::SerializedUserPreferences;
pub use profile::SerializedUserProfile;
pub use status::SerializedUserStatus;

use domain::UserId;
use serde::Serialize;

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
