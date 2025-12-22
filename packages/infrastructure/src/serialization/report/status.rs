use std::fmt;

use domain::ReportStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedReportStatus {
    Draft,
    Submitted,
    InReview,
    Approved,
    Rejected,
    Archived,
}

impl fmt::Display for SerializedReportStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerializedReportStatus::Draft => write!(f, "draft"),
            SerializedReportStatus::Submitted => write!(f, "submitted"),
            SerializedReportStatus::InReview => write!(f, "in_review"),
            SerializedReportStatus::Approved => write!(f, "approved"),
            SerializedReportStatus::Rejected => write!(f, "rejected"),
            SerializedReportStatus::Archived => write!(f, "archived"),
        }
    }
}

impl From<ReportStatus> for SerializedReportStatus {
    fn from(value: ReportStatus) -> Self {
        match value {
            ReportStatus::Draft => Self::Draft,
            ReportStatus::Submitted => Self::Submitted,
            ReportStatus::InReview => Self::InReview,
            ReportStatus::Approved => Self::Approved,
            ReportStatus::Rejected => Self::Rejected,
            ReportStatus::Archived => Self::Archived,
        }
    }
}

impl From<SerializedReportStatus> for ReportStatus {
    fn from(value: SerializedReportStatus) -> Self {
        match value {
            SerializedReportStatus::Draft => Self::Draft,
            SerializedReportStatus::Submitted => Self::Submitted,
            SerializedReportStatus::InReview => Self::InReview,
            SerializedReportStatus::Approved => Self::Approved,
            SerializedReportStatus::Rejected => Self::Rejected,
            SerializedReportStatus::Archived => Self::Archived,
        }
    }
}
