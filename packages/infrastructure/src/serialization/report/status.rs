use std::fmt;

use domain::ReportStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfrastructureReportStatus {
    Draft,
    Submitted,
    InReview,
    Approved,
    Rejected,
    Archived,
}

impl fmt::Display for InfrastructureReportStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfrastructureReportStatus::Draft => write!(f, "draft"),
            InfrastructureReportStatus::Submitted => write!(f, "submitted"),
            InfrastructureReportStatus::InReview => write!(f, "in_review"),
            InfrastructureReportStatus::Approved => write!(f, "approved"),
            InfrastructureReportStatus::Rejected => write!(f, "rejected"),
            InfrastructureReportStatus::Archived => write!(f, "archived"),
        }
    }
}

impl From<ReportStatus> for InfrastructureReportStatus {
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

impl From<InfrastructureReportStatus> for ReportStatus {
    fn from(value: InfrastructureReportStatus) -> Self {
        match value {
            InfrastructureReportStatus::Draft => Self::Draft,
            InfrastructureReportStatus::Submitted => Self::Submitted,
            InfrastructureReportStatus::InReview => Self::InReview,
            InfrastructureReportStatus::Approved => Self::Approved,
            InfrastructureReportStatus::Rejected => Self::Rejected,
            InfrastructureReportStatus::Archived => Self::Archived,
        }
    }
}
