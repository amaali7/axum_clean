pub mod content;
pub mod report;
pub mod report_type;
pub mod status;

use application::ports::report::ReportQueryResult;
use report::InfrastructureReport;
pub use status::InfrastructureReportStatus;

use domain::{Event, Report, ReportId};
use serde::{Deserialize, Serialize};
use surrealdb::Response;

use crate::error::{InfrastructureError, InfrastructureResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfrastructureReportId(String);

impl InfrastructureReportId {
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
impl Serialize for InfrastructureReportId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

// impl<'de> Deserialize<'de> for InfrastructureReportId {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let record_id = String::deserialize(deserializer)?;
//         Ok(InfrastructureReportId(record_id))
//     }
// }

impl From<ReportId> for InfrastructureReportId {
    fn from(value: ReportId) -> Self {
        Self::new(&value.id())
    }
}

impl From<InfrastructureReportId> for ReportId {
    fn from(value: InfrastructureReportId) -> Self {
        Self::new(&value.id())
    }
}

impl Event for InfrastructureReport {
    fn get_type(&self) -> &str {
        "REPORT"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InfrastructureReportQueryResult {
    Single(InfrastructureReport),
    Array(Vec<InfrastructureReport>),
    None,
}

impl InfrastructureReportQueryResult {
    pub fn get_array(&self) -> Option<Vec<InfrastructureReport>> {
        match self {
            InfrastructureReportQueryResult::Array(users) => Some(users.to_vec()),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<InfrastructureReport> {
        match self {
            InfrastructureReportQueryResult::Single(user) => Some(user.clone()),
            _ => None,
        }
    }
}

// Extension trait for SurrealDB Response
pub trait SurrealReportResponseExt {
    async fn into_report_result(self) -> Result<InfrastructureReportQueryResult, surrealdb::Error>;
}

impl SurrealReportResponseExt for Response {
    async fn into_report_result(mut self) -> Result<InfrastructureReportQueryResult, surrealdb::Error> {
        // Check if we have any results
        let num_statements = self.num_statements();
        
        if num_statements == 0 {
            return Ok(InfrastructureReportQueryResult::None);
        }

        // Try to take the first statement result as a single value
        if let Ok(single) = self.take::<Option<InfrastructureReport>>(0) {
            if let Some(user) = single {
                return Ok(InfrastructureReportQueryResult::Single(user));
            }
        }

        // Try as an array
        if let Ok(array) = self.take::<Vec<InfrastructureReport>>(0) {
            if array.is_empty() {
                return Ok(InfrastructureReportQueryResult::None);
            }
            return Ok(InfrastructureReportQueryResult::Array(array));
        }

        Ok(InfrastructureReportQueryResult::None)
    }
}

impl TryFrom<ReportQueryResult> for InfrastructureReportQueryResult {
    type Error = InfrastructureError;

    fn try_from(value: ReportQueryResult) -> InfrastructureResult<Self> {
        Ok(match value {
            ReportQueryResult::Single(report ) => Self::Single(report.try_into()?),
            ReportQueryResult::Array(reports) => {
                let mut vec_reports: Vec<InfrastructureReport> = Vec::new();
                for report in reports {
                    vec_reports.push(report.try_into()?);
                }
                Self::Array(vec_reports)
            }
            ReportQueryResult::None => Self::None,
        })
    }
}

impl TryFrom<InfrastructureReportQueryResult> for ReportQueryResult {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureReportQueryResult) -> InfrastructureResult<Self> {
        Ok(match value {
            InfrastructureReportQueryResult::Single(report) => Self::Single(report.try_into()?),
            InfrastructureReportQueryResult::Array(reports) => {
                let mut vec_reports: Vec<Report> = Vec::new();
                for report in reports {
                    vec_reports.push(report.try_into()?);
                }
                Self::Array(vec_reports)
            }
            InfrastructureReportQueryResult::None => Self::None,
        })
    }
}

