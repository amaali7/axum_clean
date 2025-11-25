pub mod content;
pub mod form;
pub mod report_type;
pub mod status;

pub use content::ReportContent;
pub use report_type::ReportType;
pub use status::ReportStatus;

use crate::DomainError;

use super::events::DomainEvent;
use super::events::ReportEvent;
use super::role::Permission;
use super::user::UserId;
use chrono::{DateTime, Utc};
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReportId(uuid::Uuid);

impl ReportId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::ops::Deref for ReportId {
    type Target = uuid::Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ReportId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct Report {
    id: ReportId,
    title: String,
    content: ReportContent,
    report_type: ReportType,
    permissions: HashSet<Permission>,
    status: ReportStatus,
    author_id: UserId,
    assigned_reviewer_id: Option<HashSet<UserId>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    due_date: Option<DateTime<Utc>>,
    version: u64,
    events: VecDeque<Box<dyn DomainEvent>>,
}

impl Report {
    pub fn create(
        title: String,
        author_id: UserId,
    ) -> Result<ReportBuilder, super::error::DomainError> {
        if title.trim().is_empty() {
            return Err(super::error::DomainError::ValidationError(
                "Report title cannot be empty".to_string(),
            ));
        }
        let report = ReportBuilder::new(author_id, None);
        Ok(report)
    }
}

#[derive(Debug)]
pub struct ReportBuilder {
    title: Option<String>,
    permissions: HashSet<Permission>,
    content: Option<ReportContent>,
    report_type: Option<ReportType>,
    author_id: UserId,
    reviewer_id: HashSet<UserId>,
    created_at: Option<DateTime<Utc>>,
    due: Option<DateTime<Utc>>,
    events: VecDeque<Box<dyn DomainEvent>>,
}

impl ReportBuilder {
    fn new(author_id: UserId, created_at: Option<DateTime<Utc>>) -> Self {
        Self {
            title: None,
            permissions: HashSet::new(),
            content: None,
            report_type: None,
            author_id,
            reviewer_id: HashSet::new(),
            created_at,
            due: None,
            events: VecDeque::new(),
        }
    }
    fn set_due(mut self, due: DateTime<Utc>) -> Self {
        self.due = Some(due);
        self
    }
    fn set_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
    fn add_permission(mut self, permission: Permission) -> Self {
        self.permissions.insert(permission);
        self
    }
    fn set_content(mut self, content: ReportContent) -> Self {
        self.content = Some(content);
        self
    }
    fn set_report_type(mut self, report_type: ReportType) -> Self {
        self.report_type = Some(report_type);
        self
    }
    fn add_reviewer(mut self, reviewer: UserId) -> Self {
        self.reviewer_id.insert(reviewer);
        self
    }
    fn add_event(mut self, event: ReportEvent) -> Self {
        self.events.push_back(Box::new(event));
        self
    }
    fn build(self) -> Result<Report, DomainError> {
        Ok(Report {
            id: ReportId::new(),
            title: self.title.ok_or(DomainError::ValidationError(
                "Report Title is require !".to_string(),
            ))?,
            content: self.content.unwrap_or(ReportContent::new(String::new())),
            report_type: self.report_type.unwrap_or(ReportType::Other),
            permissions: self.permissions,
            status: ReportStatus::Draft,
            author_id: self.author_id,
            assigned_reviewer_id: Some(self.reviewer_id),
            created_at: self.created_at.unwrap_or(Utc::now()),
            updated_at: Utc::now(),
            due_date: self.due,
            version: 1,
            events: self.events,
        })
    }
}
