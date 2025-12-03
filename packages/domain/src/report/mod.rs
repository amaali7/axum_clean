pub mod content;
pub mod form;
pub mod report_type;
pub mod status;

pub use content::ReportContent;
pub use report_type::ReportType;
pub use status::ReportStatus;

use crate::error::DomainResult;
use crate::value_objects::DateTime;
use crate::value_objects::Title;

use super::events::DomainEventId;
use super::role::Permission;
use super::user::UserId;
use std::collections::HashSet;

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
impl std::fmt::Display for ReportId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Report {
    id: ReportId,
    title: Title,
    content: ReportContent,
    report_type: ReportType,
    permissions: HashSet<Permission>,
    status: ReportStatus,
    author_id: UserId,
    assigned_reviewer_id: HashSet<UserId>,
    created_at: DateTime,
    updated_at: DateTime,
    due_date: Option<DateTime>,
    version: u64,
    events: Vec<DomainEventId>,
}

impl Report {
    pub fn new(id: ReportId, author_id: UserId) -> ReportBuilder {
        ReportBuilder::new(id, author_id)
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

    pub fn assigned_reviewer_id(&self) -> HashSet<UserId> {
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

    pub fn events(&self) -> Vec<DomainEventId> {
        self.events.clone()
    }

    pub fn version(&self) -> u64 {
        self.version.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ReportBuilder {
    id: ReportId,
    permissions: HashSet<Permission>,
    content: Option<ReportContent>,
    report_type: Option<ReportType>,
    author_id: UserId,
    reviewer_id: HashSet<UserId>,
    created_at: Option<DateTime>,
    due: Option<DateTime>,
    events: Vec<DomainEventId>,
}

impl ReportBuilder {
    pub fn new(id: ReportId, author_id: UserId) -> Self {
        Self {
            permissions: HashSet::new(),
            content: None,
            report_type: None,
            author_id,
            reviewer_id: HashSet::new(),
            created_at: None,
            due: None,
            events: Vec::new(),
            id,
        }
    }
    pub fn set_due(&mut self, due: DateTime) -> &mut Self {
        self.due = Some(due);
        self
    }
    pub fn add_permission(&mut self, permission: Permission) -> &mut Self {
        self.permissions.insert(permission);
        self
    }
    pub fn set_content(&mut self, content: ReportContent) -> &mut Self {
        self.content = Some(content);
        self
    }
    pub fn set_report_type(&mut self, report_type: ReportType) -> &mut Self {
        self.report_type = Some(report_type);
        self
    }
    pub fn add_reviewer(&mut self, reviewer: UserId) -> &mut Self {
        self.reviewer_id.insert(reviewer);
        self
    }
    pub fn add_event(&mut self, event: DomainEventId) -> &mut Self {
        self.events.push(event);
        self
    }
    pub fn build(self, title: &str, time: DateTime) -> DomainResult<Report> {
        Ok(Report {
            id: self.id,
            title: Title::new(title)?,
            content: self.content.unwrap_or(ReportContent::default()),
            report_type: self.report_type.unwrap_or(ReportType::Other),
            permissions: self.permissions,
            status: ReportStatus::Draft,
            author_id: self.author_id,
            assigned_reviewer_id: self.reviewer_id,
            created_at: self.created_at.unwrap_or(time.clone()),
            updated_at: time,
            due_date: self.due,
            version: 1,
            events: self.events,
        })
    }
}
