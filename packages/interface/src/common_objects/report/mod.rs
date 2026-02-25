pub mod report_type;
pub mod status;

use domain::ReportId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InterfaceReportId(String);

impl InterfaceReportId {
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

impl From<InterfaceReportId> for ReportId {
    fn from(value: InterfaceReportId) -> Self {
        Self::new(&value.id())
    }
}

impl From<ReportId> for InterfaceReportId {
    fn from(value: ReportId) -> Self {
        Self::new(&value.id())
    }
}
