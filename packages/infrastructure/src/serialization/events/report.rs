use domain::events::ReportEvent;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        report::SerializedReportStatus, value_objects::date_time::SerializedDateTime,
        SerializedDiff, SerializedReportId, SerializedUserId,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedReportEvent {
    ReportCreated {
        report_id: SerializedReportId,
        auther_id: SerializedUserId,
        occurred_at: SerializedDateTime,
    },
    ReportRemoved {
        report_id: SerializedReportId,
        auther_id: SerializedUserId,
        occurred_at: SerializedDateTime,
    },
    ReportModified {
        report_id: SerializedReportId,
        auther_id: SerializedUserId,
        diff: SerializedDiff,
        occurred_at: SerializedDateTime,
    },
    ReportStatusChanged {
        report_id: SerializedReportId,
        old_status: SerializedReportStatus,
        new_status: SerializedReportStatus,
        changed_by: SerializedUserId,
        occurred_at: SerializedDateTime,
    },
}

impl SerializedReportEvent {
    pub fn event_type(&self) -> &'static str {
        match self {
            SerializedReportEvent::ReportCreated { .. } => "report.created",
            SerializedReportEvent::ReportRemoved { .. } => "report.removed",
            SerializedReportEvent::ReportModified { .. } => "report.modified",
            SerializedReportEvent::ReportStatusChanged { .. } => "report.status.changed",
        }
    }
    pub fn occurred_at(&self) -> SerializedDateTime {
        match self {
            SerializedReportEvent::ReportCreated { occurred_at, .. } => occurred_at.clone(),
            SerializedReportEvent::ReportRemoved { occurred_at, .. } => occurred_at.clone(),
            SerializedReportEvent::ReportModified { occurred_at, .. } => occurred_at.clone(),
            SerializedReportEvent::ReportStatusChanged { occurred_at, .. } => occurred_at.clone(),
        }
    }
}

impl TryFrom<ReportEvent> for SerializedReportEvent {
    type Error = InfrastructureError;

    fn try_from(value: ReportEvent) -> InfrastructureResult<Self> {
        Ok(match value {
            ReportEvent::ReportCreated {
                report_id,
                auther_id,
                occurred_at,
            } => Self::ReportCreated {
                report_id: report_id.into(),
                auther_id: auther_id.into(),
                occurred_at: occurred_at.try_into()?,
            },
            ReportEvent::ReportRemoved {
                report_id,
                auther_id,
                occurred_at,
            } => Self::ReportRemoved {
                report_id: report_id.into(),
                auther_id: auther_id.into(),
                occurred_at: occurred_at.try_into()?,
            },
            ReportEvent::ReportModified {
                report_id,
                auther_id,
                diff,
                occurred_at,
            } => Self::ReportModified {
                report_id: report_id.into(),
                auther_id: auther_id.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            ReportEvent::ReportStatusChanged {
                report_id,
                old_status,
                new_status,
                changed_by,
                occurred_at,
            } => Self::ReportStatusChanged {
                report_id: report_id.into(),
                old_status: old_status.into(),
                new_status: new_status.into(),
                changed_by: changed_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
        })
    }
}

impl TryFrom<SerializedReportEvent> for ReportEvent {
    type Error = InfrastructureError;

    fn try_from(value: SerializedReportEvent) -> InfrastructureResult<Self> {
        Ok(match value {
            SerializedReportEvent::ReportCreated {
                report_id,
                auther_id,
                occurred_at,
            } => Self::ReportCreated {
                report_id: report_id.into(),
                auther_id: auther_id.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedReportEvent::ReportRemoved {
                report_id,
                auther_id,
                occurred_at,
            } => Self::ReportRemoved {
                report_id: report_id.into(),
                auther_id: auther_id.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedReportEvent::ReportModified {
                report_id,
                auther_id,
                diff,
                occurred_at,
            } => Self::ReportModified {
                report_id: report_id.into(),
                auther_id: auther_id.into(),
                diff: diff.into(),
                occurred_at: occurred_at.try_into()?,
            },
            SerializedReportEvent::ReportStatusChanged {
                report_id,
                old_status,
                new_status,
                changed_by,
                occurred_at,
            } => Self::ReportStatusChanged {
                report_id: report_id.into(),
                old_status: old_status.into(),
                new_status: new_status.into(),
                changed_by: changed_by.into(),
                occurred_at: occurred_at.try_into()?,
            },
        })
    }
}
