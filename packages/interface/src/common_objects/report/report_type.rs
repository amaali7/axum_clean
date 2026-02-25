use axum::Form;
use domain::ReportType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterfaceReportType {
    Financial,
    Technical,
    Progress,
    Incident,
    Audit,
    Other,
}

impl InterfaceReportType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Financial,
            Self::Technical,
            Self::Progress,
            Self::Incident,
            Self::Audit,
            Self::Other,
        ]
    }

    pub fn requires_approval(&self) -> bool {
        matches!(self, Self::Financial | Self::Audit)
    }
}

impl From<ReportType> for InterfaceReportType {
    fn from(value: ReportType) -> Self {
        match value {
            ReportType::Financial => InterfaceReportType::Financial,
            ReportType::Technical => InterfaceReportType::Technical,
            ReportType::Progress => InterfaceReportType::Progress,
            ReportType::Incident => InterfaceReportType::Incident,
            ReportType::Audit => InterfaceReportType::Audit,
            ReportType::Other => InterfaceReportType::Other,
        }
    }
}

impl From<InterfaceReportType> for ReportType {
    fn from(value: InterfaceReportType) -> Self {
        match value {
            InterfaceReportType::Financial => Self::Financial,
            InterfaceReportType::Technical => Self::Technical,
            InterfaceReportType::Progress => Self::Progress,
            InterfaceReportType::Incident => Self::Incident,
            InterfaceReportType::Audit => Self::Audit,
            InterfaceReportType::Other => Self::Other,
        }
    }
}
