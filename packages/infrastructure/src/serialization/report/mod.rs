pub mod content;
pub mod report;
pub mod report_type;
pub mod status;

use application::ports::report::ReportQueryResult;
use report::SerializedReport;
pub use status::SerializedReportStatus;

use domain::{Event, Report, ReportId};
use serde::{Deserialize, Serialize};
use surrealdb::Response;

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl Event for SerializedReport {
    fn get_type(&self) -> &str {
        "REPORT"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SerializedReportQueryResult {
    Single(SerializedReport),
    Array(Vec<SerializedReport>),
    None,
}

impl SerializedReportQueryResult {
    pub fn get_array(&self) -> Option<Vec<SerializedReport>> {
        match self {
            SerializedReportQueryResult::Array(users) => Some(users.to_vec()),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<SerializedReport> {
        match self {
            SerializedReportQueryResult::Single(user) => Some(user.clone()),
            _ => None,
        }
    }
}

// Extension trait for SurrealDB Response
pub trait SurrealReportResponseExt {
    async fn into_report_result(self) -> Result<SerializedReportQueryResult, surrealdb::Error>;
}

impl SurrealReportResponseExt for Response {
    async fn into_report_result(mut self) -> Result<SerializedReportQueryResult, surrealdb::Error> {
        // Check if we have any results
        let num_statements = self.num_statements();
        
        if num_statements == 0 {
            return Ok(SerializedReportQueryResult::None);
        }

        // Try to take the first statement result as a single value
        if let Ok(single) = self.take::<Option<SerializedReport>>(0) {
            if let Some(user) = single {
                return Ok(SerializedReportQueryResult::Single(user));
            }
        }

        // Try as an array
        if let Ok(array) = self.take::<Vec<SerializedReport>>(0) {
            if array.is_empty() {
                return Ok(SerializedReportQueryResult::None);
            }
            return Ok(SerializedReportQueryResult::Array(array));
        }

        Ok(SerializedReportQueryResult::None)
    }
}

impl TryFrom<ReportQueryResult> for SerializedReportQueryResult {
    type Error = InfrastructureError;

    fn try_from(value: ReportQueryResult) -> InfrastructureResult<Self> {
        Ok(match value {
            ReportQueryResult::Single(report ) => Self::Single(report.try_into()?),
            ReportQueryResult::Array(reports) => {
                let mut vec_reports: Vec<SerializedReport> = Vec::new();
                for report in reports {
                    vec_reports.push(report.try_into()?);
                }
                Self::Array(vec_reports)
            }
            ReportQueryResult::None => Self::None,
        })
    }
}

impl TryFrom<SerializedReportQueryResult> for ReportQueryResult {
    type Error = InfrastructureError;

    fn try_from(value: SerializedReportQueryResult) -> InfrastructureResult<Self> {
        Ok(match value {
            SerializedReportQueryResult::Single(report) => Self::Single(report.try_into()?),
            SerializedReportQueryResult::Array(reports) => {
                let mut vec_reports: Vec<Report> = Vec::new();
                for report in reports {
                    vec_reports.push(report.try_into()?);
                }
                Self::Array(vec_reports)
            }
            SerializedReportQueryResult::None => Self::None,
        })
    }
}

