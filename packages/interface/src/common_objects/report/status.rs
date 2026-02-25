use std::fmt;

use domain::ReportStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterfaceReportStatus {
    Draft,
    Submitted,
    InReview,
    Approved,
    Rejected,
    Archived,
}

impl fmt::Display for InterfaceReportStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterfaceReportStatus::Draft => write!(f, "draft"),
            InterfaceReportStatus::Submitted => write!(f, "submitted"),
            InterfaceReportStatus::InReview => write!(f, "in_review"),
            InterfaceReportStatus::Approved => write!(f, "approved"),
            InterfaceReportStatus::Rejected => write!(f, "rejected"),
            InterfaceReportStatus::Archived => write!(f, "archived"),
        }
    }
}

impl From<InterfaceReportStatus> for ReportStatus {
    fn from(value: InterfaceReportStatus) -> Self {
        match value {
            InterfaceReportStatus::Draft => Self::Draft,
            InterfaceReportStatus::Submitted => Self::Submitted,
            InterfaceReportStatus::InReview => Self::InReview,
            InterfaceReportStatus::Approved => Self::Approved,
            InterfaceReportStatus::Rejected => Self::Rejected,
            InterfaceReportStatus::Archived => Self::Submitted,
        }
    }
}

impl From<ReportStatus> for InterfaceReportStatus {
    fn from(value: ReportStatus) -> Self {
        match value {
            ReportStatus::Draft => Self::Draft,
            ReportStatus::Submitted => Self::Submitted,
            ReportStatus::InReview => Self::InReview,
            ReportStatus::Approved => Self::Approved,
            ReportStatus::Rejected => Self::Rejected,
            ReportStatus::Archived => Self::Submitted,
        }
    }
}
