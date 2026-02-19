use std::collections::HashSet;

use domain::Report;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::{SerializedDateTime, SerializedTitle},
        SerializedPermission, SerializedUserId,
    },
};

use super::{
    content::SerializedReportContent, report_type::SerializedReportType, SerializedReportId,
    SerializedReportStatus,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedReport {
    id: SerializedReportId,
    title: SerializedTitle,
    content: SerializedReportContent,
    report_type: SerializedReportType,
    permissions: HashSet<SerializedPermission>,
    status: SerializedReportStatus,
    author_id: SerializedUserId,
    assigned_reviewer_id: HashSet<SerializedUserId>,
    created_at: SerializedDateTime,
    updated_at: SerializedDateTime,
    due_date: Option<SerializedDateTime>,
    version: u64,
}

impl SerializedReport {
    pub fn new(id: SerializedReportId, author_id: SerializedUserId) -> SerializedReportBuilder {
        SerializedReportBuilder::new(id, author_id)
    }

    pub fn id(&self) -> SerializedReportId {
        self.id.clone()
    }

    pub fn title(&self) -> SerializedTitle {
        self.title.clone()
    }

    pub fn content(&self) -> SerializedReportContent {
        self.content.clone()
    }

    pub fn report_type(&self) -> SerializedReportType {
        self.report_type.clone()
    }

    pub fn permissions(&self) -> HashSet<SerializedPermission> {
        self.permissions.clone()
    }

    pub fn status(&self) -> SerializedReportStatus {
        self.status.clone()
    }

    pub fn author_id(&self) -> SerializedUserId {
        self.author_id.clone()
    }

    pub fn assigned_reviewer_id(&self) -> HashSet<SerializedUserId> {
        self.assigned_reviewer_id.clone()
    }

    pub fn created_at(&self) -> SerializedDateTime {
        self.created_at.clone()
    }

    pub fn updated_at(&self) -> SerializedDateTime {
        self.updated_at.clone()
    }

    pub fn due_date(&self) -> Option<SerializedDateTime> {
        self.due_date.clone()
    }

    pub fn version(&self) -> u64 {
        self.version.clone()
    }
}

#[derive(Debug, Clone)]
pub struct SerializedReportBuilder {
    id: SerializedReportId,
    permissions: HashSet<SerializedPermission>,
    content: Option<SerializedReportContent>,
    report_type: Option<SerializedReportType>,
    author_id: SerializedUserId,
    status: SerializedReportStatus,
    reviewer_id: HashSet<SerializedUserId>,
    created_at: Option<SerializedDateTime>,
    due: Option<SerializedDateTime>,
    version: u64,
}

impl SerializedReportBuilder {
    pub fn new(id: SerializedReportId, author_id: SerializedUserId) -> Self {
        Self {
            permissions: HashSet::new(),
            content: None,
            report_type: None,
            author_id,
            reviewer_id: HashSet::new(),
            created_at: None,
            due: None,
            id,
            status: SerializedReportStatus::Draft,
            version: 1,
        }
    }
    pub fn set_status(&mut self, status: SerializedReportStatus) -> &mut Self {
        self.status = status;
        self
    }
    pub fn set_created_at(&mut self, created_at: SerializedDateTime) -> &mut Self {
        self.created_at = Some(created_at);
        self
    }
    pub fn set_version(&mut self, version: u64) -> &mut Self {
        self.version = version;
        self
    }

    pub fn set_due(&mut self, due: SerializedDateTime) -> &mut Self {
        self.due = Some(due);
        self
    }
    pub fn add_permission(&mut self, permission: SerializedPermission) -> &mut Self {
        self.permissions.insert(permission);
        self
    }
    pub fn set_content(&mut self, content: SerializedReportContent) -> &mut Self {
        self.content = Some(content);
        self
    }
    pub fn set_report_type(&mut self, report_type: SerializedReportType) -> &mut Self {
        self.report_type = Some(report_type);
        self
    }
    pub fn add_reviewer(&mut self, reviewer: SerializedUserId) -> &mut Self {
        self.reviewer_id.insert(reviewer);
        self
    }

    pub fn build(
        self,
        title: &str,
        updated_at: SerializedDateTime,
    ) -> InfrastructureResult<SerializedReport> {
        Ok(SerializedReport {
            id: self.id,
            title: SerializedTitle::new(title)?,
            content: self.content.unwrap_or(SerializedReportContent::default()),
            report_type: self.report_type.unwrap_or(SerializedReportType::Other),
            permissions: self.permissions,
            status: self.status,
            author_id: self.author_id,
            assigned_reviewer_id: self.reviewer_id,
            created_at: self.created_at.unwrap_or(updated_at.clone()),
            updated_at: updated_at,
            due_date: self.due,
            version: self.version,
        })
    }
}

impl TryFrom<Report> for SerializedReport {
    type Error = InfrastructureError;

    fn try_from(value: Report) -> InfrastructureResult<Self> {
        let mut report_builder = Self::new(value.id().into(), value.author_id().into());
        report_builder
            .set_content(value.content().try_into()?)
            .set_report_type(value.report_type().into())
            .set_status(value.status().into())
            .set_version(value.version());
        match value.due_date() {
            Some(due) => {
                report_builder.set_due(due.try_into()?);
            }
            None => (),
        }
        for permission in value.permissions().into_iter() {
            report_builder.add_permission(permission.into());
        }
        for reviewer in value.assigned_reviewer_id().into_iter() {
            report_builder.add_reviewer(reviewer.into());
        }

        report_builder.build(&value.title().title(), value.updated_at().try_into()?)
    }
}

impl TryFrom<SerializedReport> for Report {
    type Error = InfrastructureError;

    fn try_from(value: SerializedReport) -> InfrastructureResult<Self> {
        let mut report_builder = Self::new(value.id().into(), value.author_id().into());
        report_builder
            .set_content(value.content().try_into()?)
            .set_report_type(value.report_type().into())
            .set_status(value.status().into())
            .set_version(value.version());
        match value.due_date() {
            Some(due) => {
                report_builder.set_due(due.try_into()?);
            }
            None => (),
        }
        for permission in value.permissions().into_iter() {
            report_builder.add_permission(permission.into());
        }
        for reviewer in value.assigned_reviewer_id().into_iter() {
            report_builder.add_reviewer(reviewer.into());
        }

        report_builder
            .build(&value.title().title(), value.updated_at().try_into()?)
            .map_err(|err| InfrastructureError::Domain(err))
    }
}
