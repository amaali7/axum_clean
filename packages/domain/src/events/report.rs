use crate::{value_objects::Diff, ReportId, ReportStatus, UserId};

#[derive(Debug, Clone)]
pub enum ReportEvent {
    ReportCreated {
        report_id: ReportId,
        auther_id: UserId,
    },
    ReportRemoved {
        report_id: ReportId,
        removed_by: UserId,
    },
    ReportModified {
        report_id: ReportId,
        auther_id: UserId,
        diff: Diff,
    },
    ReportStatusChanged {
        report_id: ReportId,
        old_status: ReportStatus,
        new_status: ReportStatus,
        changed_by: UserId,
    },
}

impl ReportEvent {
    fn event_type(&self) -> &'static str {
        match self {
            ReportEvent::ReportCreated { .. } => "report.created",
            ReportEvent::ReportRemoved { .. } => "report.removed",
            ReportEvent::ReportModified { .. } => "report.modified",
            ReportEvent::ReportStatusChanged { .. } => "report.status.changed",
        }
    }
}
