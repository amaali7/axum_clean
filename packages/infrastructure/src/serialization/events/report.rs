use domain::events::ReportEvent;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        report::SerializedReportStatus, SerializedDiff, SerializedReportId, SerializedUserId,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedReportEvent {
    ReportCreated {
        report_id: SerializedReportId,
        auther_id: SerializedUserId,
    },
    ReportRemoved {
        report_id: SerializedReportId,
        removed_by: SerializedUserId,
    },
    ReportModified {
        report_id: SerializedReportId,
        auther_id: SerializedUserId,
        diff: SerializedDiff,
    },
    ReportStatusChanged {
        report_id: SerializedReportId,
        old_status: SerializedReportStatus,
        new_status: SerializedReportStatus,
        changed_by: SerializedUserId,
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
}

impl TryFrom<ReportEvent> for SerializedReportEvent {
    type Error = InfrastructureError;

    fn try_from(value: ReportEvent) -> InfrastructureResult<Self> {
        Ok(match value {
            ReportEvent::ReportCreated {
                report_id,
                auther_id,
            } => Self::ReportCreated {
                report_id: report_id.into(),
                auther_id: auther_id.into(),
            },
            ReportEvent::ReportRemoved {
                report_id,
                removed_by,
            } => Self::ReportRemoved {
                report_id: report_id.into(),
                removed_by: removed_by.into(),
            },
            ReportEvent::ReportModified {
                report_id,
                auther_id,
                diff,
            } => Self::ReportModified {
                report_id: report_id.into(),
                auther_id: auther_id.into(),
                diff: diff.into(),
            },
            ReportEvent::ReportStatusChanged {
                report_id,
                old_status,
                new_status,
                changed_by,
            } => Self::ReportStatusChanged {
                report_id: report_id.into(),
                old_status: old_status.into(),
                new_status: new_status.into(),
                changed_by: changed_by.into(),
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
            } => Self::ReportCreated {
                report_id: report_id.into(),
                auther_id: auther_id.into(),
            },
            SerializedReportEvent::ReportRemoved {
                report_id,
                removed_by,
            } => Self::ReportRemoved {
                report_id: report_id.into(),
                removed_by: removed_by.into(),
            },
            SerializedReportEvent::ReportModified {
                report_id,
                auther_id,
                diff,
            } => Self::ReportModified {
                report_id: report_id.into(),
                auther_id: auther_id.into(),
                diff: diff.into(),
            },
            SerializedReportEvent::ReportStatusChanged {
                report_id,
                old_status,
                new_status,
                changed_by,
            } => Self::ReportStatusChanged {
                report_id: report_id.into(),
                old_status: old_status.into(),
                new_status: new_status.into(),
                changed_by: changed_by.into(),
            },
        })
    }
}
