use chrono::{DateTime, Utc};

use crate::{value_objects::Diff, ReportId, ReportStatus, UserId};

use super::DomainEvent;

#[derive(Debug, Clone)]
pub enum ReportEvent {
    ReportCreated {
        report_id: ReportId,
        auther_id: UserId,
        occurred_at: DateTime<Utc>,
    },
    ReportRemoved {
        report_id: ReportId,
        auther_id: UserId,
        occurred_at: DateTime<Utc>,
    },
    ReportModified {
        report_id: ReportId,
        auther_id: UserId,
        diff: Diff,
        occurred_at: DateTime<Utc>,
    },
    ReportStatusChanged {
        report_id: ReportId,
        old_status: ReportStatus,
        new_status: ReportStatus,
        changed_by: UserId,
        occurred_at: DateTime<Utc>,
    },
}

impl DomainEvent for ReportEvent {
    fn event_type(&self) -> &'static str {
        match self {
            ReportEvent::ReportCreated { .. } => "report.created",
            ReportEvent::ReportRemoved { .. } => "report.removed",
            ReportEvent::ReportModified { .. } => "report.modified",
            ReportEvent::ReportStatusChanged { .. } => "report.status.changed",
        }
    }
    fn occurred_at(&self) -> DateTime<Utc> {
        match self {
            ReportEvent::ReportCreated { occurred_at, .. } => *occurred_at,
            ReportEvent::ReportRemoved { occurred_at, .. } => *occurred_at,
            ReportEvent::ReportModified { occurred_at, .. } => *occurred_at,
            ReportEvent::ReportStatusChanged { occurred_at, .. } => *occurred_at,
        }
    }
}
