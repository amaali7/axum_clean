use std::fmt;

#[derive(Debug, Clone)]
pub enum ReportStatus {
    Draft,
    Submitted,
    InReview,
    Approved,
    Rejected,
    Archived,
}

impl fmt::Display for ReportStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReportStatus::Draft => write!(f, "draft"),
            ReportStatus::Submitted => write!(f, "submitted"),
            ReportStatus::InReview => write!(f, "in_review"),
            ReportStatus::Approved => write!(f, "approved"),
            ReportStatus::Rejected => write!(f, "rejected"),
            ReportStatus::Archived => write!(f, "archived"),
        }
    }
}
