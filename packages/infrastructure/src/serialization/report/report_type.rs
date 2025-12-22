use domain::ReportType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializedReportType {
    Financial,
    Technical,
    Progress,
    Incident,
    Audit,
    Other,
}

impl SerializedReportType {
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

impl From<ReportType> for SerializedReportType {
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

impl From<SerializedReportType> for ReportType {
    fn from(value: SerializedReportType) -> Self {
        match value {
            SerializedReportType::Financial => Self::Financial,
            SerializedReportType::Technical => Self::Technical,
            SerializedReportType::Progress => Self::Progress,
            SerializedReportType::Incident => Self::Incident,
            SerializedReportType::Audit => Self::Audit,
            SerializedReportType::Other => Self::Other,
        }
    }
}
