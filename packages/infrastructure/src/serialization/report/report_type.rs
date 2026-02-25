use domain::ReportType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfrastructureReportType {
    Financial,
    Technical,
    Progress,
    Incident,
    Audit,
    Other,
}

impl InfrastructureReportType {
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

impl From<ReportType> for InfrastructureReportType {
    fn from(value: ReportType) -> Self {
        match value {
            ReportType::Financial => Self::Financial,
            ReportType::Technical => Self::Technical,
            ReportType::Progress => Self::Progress,
            ReportType::Incident => Self::Incident,
            ReportType::Audit => Self::Audit,
            ReportType::Other => Self::Other,
        }
    }
}

impl From<InfrastructureReportType> for ReportType {
    fn from(value: InfrastructureReportType) -> Self {
        match value {
            InfrastructureReportType::Financial => Self::Financial,
            InfrastructureReportType::Technical => Self::Technical,
            InfrastructureReportType::Progress => Self::Progress,
            InfrastructureReportType::Incident => Self::Incident,
            InfrastructureReportType::Audit => Self::Audit,
            InfrastructureReportType::Other => Self::Other,
        }
    }
}
