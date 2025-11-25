#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportType {
    Financial,
    Technical,
    Progress,
    Incident,
    Audit,
    Other,
}

impl ReportType {
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
