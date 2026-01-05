pub mod content;
pub mod report;
pub mod report_type;
pub mod status;

pub use status::SerializedReportStatus;

use domain::ReportId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct SerializedReportId(String);

impl SerializedReportId {
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
impl Serialize for SerializedReportId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// impl<'de> Deserialize<'de> for SerializedReportId {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let record_id = String::deserialize(deserializer)?;
//         Ok(SerializedReportId(record_id))
//     }
// }

impl From<ReportId> for SerializedReportId {
    fn from(value: ReportId) -> Self {
        Self::new(&value.id())
    }
}

impl From<SerializedReportId> for ReportId {
    fn from(value: SerializedReportId) -> Self {
        Self::new(&value.id())
    }
}
