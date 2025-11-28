pub mod content;
pub mod form;
pub mod report_type;
pub mod status;

pub use content::ReportContent;
pub use report_type::ReportType;
pub use status::ReportStatus;

use crate::value_objects::Body;
use crate::value_objects::DateTime;
use crate::value_objects::Title;
use crate::DomainError;

use super::events::DomainEvent;
use super::role::Permission;
use super::user::UserId;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReportId(String);

impl ReportId {
    pub fn new() -> Self {
        Self(String::new())
    }
}

impl std::ops::Deref for ReportId {
    type Target = String;
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
    title: Title,
    content: ReportContent,
    report_type: ReportType,
    permissions: HashSet<Permission>,
    status: ReportStatus,
    author_id: UserId,
    assigned_reviewer_id: Option<HashSet<UserId>>,
    created_at: DateTime,
    updated_at: DateTime,
    due_date: Option<DateTime>,
    version: u64,
    events: VecDeque<DomainEvent>,
}

impl Report {
    pub fn new(
        id: ReportId,
        title: String,
        author_id: UserId,
    ) -> Result<ReportBuilder, super::error::DomainError> {
        if title.trim().is_empty() {
            return Err(super::error::DomainError::ValidationError(
                "Report title cannot be empty".to_string(),
            ));
        }
        let report = ReportBuilder::new(id, author_id, None);
        Ok(report)
    }

    pub fn id(&self) -> ReportId {
        self.id.clone()
    }

    pub fn title(&self) -> Title {
        self.title.clone()
    }

    pub fn content(&self) -> ReportContent {
        self.content.clone()
    }

    pub fn report_type(&self) -> ReportType {
        self.report_type.clone()
    }

    pub fn permissions(&self) -> HashSet<Permission> {
        self.permissions.clone()
    }

    pub fn status(&self) -> ReportStatus {
        self.status.clone()
    }

    pub fn author_id(&self) -> UserId {
        self.author_id.clone()
    }

    pub fn assigned_reviewer_id(&self) -> Option<HashSet<UserId>> {
        self.assigned_reviewer_id.clone()
    }

    pub fn created_at(&self) -> DateTime {
        self.created_at.clone()
    }

    pub fn updated_at(&self) -> DateTime {
        self.updated_at.clone()
    }

    pub fn due_date(&self) -> Option<DateTime> {
        self.due_date.clone()
    }

    pub fn events(&self) -> VecDeque<DomainEvent> {
        self.events.clone()
    }

    pub fn version(&self) -> u64 {
        self.version.clone()
    }
}

#[derive(Debug)]
pub struct ReportBuilder {
    id: ReportId,
    permissions: HashSet<Permission>,
    content: Option<ReportContent>,
    report_type: Option<ReportType>,
    author_id: UserId,
    reviewer_id: HashSet<UserId>,
    created_at: Option<DateTime>,
    due: Option<DateTime>,
    events: VecDeque<DomainEvent>,
}

impl ReportBuilder {
    fn new(id: ReportId, author_id: UserId, created_at: Option<DateTime>) -> Self {
        Self {
            permissions: HashSet::new(),
            content: None,
            report_type: None,
            author_id,
            reviewer_id: HashSet::new(),
            created_at,
            due: None,
            events: VecDeque::new(),
            id,
        }
    }
    fn set_due(mut self, due: DateTime) -> Self {
        self.due = Some(due);
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
    fn add_event(mut self, event: DomainEvent) -> Self {
        self.events.push_back(event);
        self
    }
    fn build(self, title: &str, time: DateTime) -> Result<Report, DomainError> {
        Ok(Report {
            id: self.id,
            title: Title::new(title)?,
            content: self
                .content
                .unwrap_or(ReportContent::new(Body::new("Empty Content")?)),
            report_type: self.report_type.unwrap_or(ReportType::Other),
            permissions: self.permissions,
            status: ReportStatus::Draft,
            author_id: self.author_id,
            assigned_reviewer_id: Some(self.reviewer_id),
            created_at: self.created_at.unwrap_or(time.clone()),
            updated_at: time,
            due_date: self.due,
            version: 1,
            events: self.events,
        })
    }
}
