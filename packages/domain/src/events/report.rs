use crate::{value_objects::Diff, DateTime, ReportId, ReportStatus, UserId};

#[derive(Debug, Clone)]
pub enum ReportEvent {
    ReportCreated {
        report_id: ReportId,
        auther_id: UserId,
        occurred_at: DateTime,
    },
    ReportRemoved {
        report_id: ReportId,
        auther_id: UserId,
        occurred_at: DateTime,
    },
    ReportModified {
        report_id: ReportId,
        auther_id: UserId,
        diff: Diff,
        occurred_at: DateTime,
    },
    ReportStatusChanged {
        report_id: ReportId,
        old_status: ReportStatus,
        new_status: ReportStatus,
        changed_by: UserId,
        occurred_at: DateTime,
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
    fn occurred_at(&self) -> DateTime {
        match self {
            ReportEvent::ReportCreated { occurred_at, .. } => occurred_at.clone(),
            ReportEvent::ReportRemoved { occurred_at, .. } => occurred_at.clone(),
            ReportEvent::ReportModified { occurred_at, .. } => occurred_at.clone(),
            ReportEvent::ReportStatusChanged { occurred_at, .. } => occurred_at.clone(),
        }
    }
}
