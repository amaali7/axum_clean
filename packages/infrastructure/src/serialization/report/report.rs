use std::collections::HashSet;

use domain::Report;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::{InfrastructureDateTime, InfrastructureTitle},
        InfrastructurePermission, InfrastructureUserId,
    },
};

use super::{
    content::InfrastructureReportContent, report_type::InfrastructureReportType, InfrastructureReportId,
    InfrastructureReportStatus,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureReport {
    id: InfrastructureReportId,
    title: InfrastructureTitle,
    content: InfrastructureReportContent,
    report_type: InfrastructureReportType,
    permissions: HashSet<InfrastructurePermission>,
    status: InfrastructureReportStatus,
    author_id: InfrastructureUserId,
    assigned_reviewer_id: HashSet<InfrastructureUserId>,
    created_at: InfrastructureDateTime,
    updated_at: InfrastructureDateTime,
    due_date: Option<InfrastructureDateTime>,
    version: u64,
}

impl InfrastructureReport {
    pub fn new(id: InfrastructureReportId, author_id: InfrastructureUserId) -> InfrastructureReportBuilder {
        InfrastructureReportBuilder::new(id, author_id)
    }

    pub fn id(&self) -> InfrastructureReportId {
        self.id.clone()
    }

    pub fn title(&self) -> InfrastructureTitle {
        self.title.clone()
    }

    pub fn content(&self) -> InfrastructureReportContent {
        self.content.clone()
    }

    pub fn report_type(&self) -> InfrastructureReportType {
        self.report_type.clone()
    }

    pub fn permissions(&self) -> HashSet<InfrastructurePermission> {
        self.permissions.clone()
    }

    pub fn status(&self) -> InfrastructureReportStatus {
        self.status.clone()
    }

    pub fn author_id(&self) -> InfrastructureUserId {
        self.author_id.clone()
    }

    pub fn assigned_reviewer_id(&self) -> HashSet<InfrastructureUserId> {
        self.assigned_reviewer_id.clone()
    }

    pub fn created_at(&self) -> InfrastructureDateTime {
        self.created_at.clone()
    }

    pub fn updated_at(&self) -> InfrastructureDateTime {
        self.updated_at.clone()
    }

    pub fn due_date(&self) -> Option<InfrastructureDateTime> {
        self.due_date.clone()
    }

    pub fn version(&self) -> u64 {
        self.version.clone()
    }
}

#[derive(Debug, Clone)]
pub struct InfrastructureReportBuilder {
    id: InfrastructureReportId,
    permissions: HashSet<InfrastructurePermission>,
    content: Option<InfrastructureReportContent>,
    report_type: Option<InfrastructureReportType>,
    author_id: InfrastructureUserId,
    status: InfrastructureReportStatus,
    reviewer_id: HashSet<InfrastructureUserId>,
    created_at: Option<InfrastructureDateTime>,
    due: Option<InfrastructureDateTime>,
    version: u64,
}

impl InfrastructureReportBuilder {
    pub fn new(id: InfrastructureReportId, author_id: InfrastructureUserId) -> Self {
        Self {
            permissions: HashSet::new(),
            content: None,
            report_type: None,
            author_id,
            reviewer_id: HashSet::new(),
            created_at: None,
            due: None,
            id,
            status: InfrastructureReportStatus::Draft,
            version: 1,
        }
    }
    pub fn set_status(&mut self, status: InfrastructureReportStatus) -> &mut Self {
        self.status = status;
        self
    }
    pub fn set_created_at(&mut self, created_at: InfrastructureDateTime) -> &mut Self {
        self.created_at = Some(created_at);
        self
    }
    pub fn set_version(&mut self, version: u64) -> &mut Self {
        self.version = version;
        self
    }

    pub fn set_due(&mut self, due: InfrastructureDateTime) -> &mut Self {
        self.due = Some(due);
        self
    }
    pub fn add_permission(&mut self, permission: InfrastructurePermission) -> &mut Self {
        self.permissions.insert(permission);
        self
    }
    pub fn set_content(&mut self, content: InfrastructureReportContent) -> &mut Self {
        self.content = Some(content);
        self
    }
    pub fn set_report_type(&mut self, report_type: InfrastructureReportType) -> &mut Self {
        self.report_type = Some(report_type);
        self
    }
    pub fn add_reviewer(&mut self, reviewer: InfrastructureUserId) -> &mut Self {
        self.reviewer_id.insert(reviewer);
        self
    }

    pub fn build(
        self,
        title: &str,
        updated_at: InfrastructureDateTime,
    ) -> InfrastructureResult<InfrastructureReport> {
        Ok(InfrastructureReport {
            id: self.id,
            title: InfrastructureTitle::new(title)?,
            content: self.content.unwrap_or(InfrastructureReportContent::default()),
            report_type: self.report_type.unwrap_or(InfrastructureReportType::Other),
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

impl TryFrom<Report> for InfrastructureReport {
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

impl TryFrom<InfrastructureReport> for Report {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureReport) -> InfrastructureResult<Self> {
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
