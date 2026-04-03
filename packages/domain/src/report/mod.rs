pub mod content;
pub mod report_type;
pub mod status;

use std::collections::HashSet;

pub use content::ReportContent;
pub use report_type::ReportType;
pub use status::ReportStatus;

use crate::error::DomainResult;
use crate::value_objects::DateTime;
use crate::value_objects::Title;
use crate::Event;
use crate::TenantId;

use super::user::UserId;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ReportId(String);

impl ReportId {
    pub fn new(id: &str) -> Self {
        Self(id.into())
    }
    pub fn id(&self) -> &str {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for ReportId {
    type Target = str;
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
    status: ReportStatus,
    author_id: UserId,
    owner_tenant: TenantId,
    shared_with_tenants: HashSet<TenantId>,
    assigned_reviewer_id: HashSet<UserId>,
    created_at: DateTime,
    updated_at: DateTime,
    due_date: Option<DateTime>,
    version: u64,
}

#[derive(Debug, Clone)]
pub struct ReportParts {
    pub id: ReportId,
    pub title: Title,
    pub content: ReportContent,
    pub report_type: ReportType,
    pub status: ReportStatus,
    pub author_id: UserId,
    pub owner_tenant: TenantId,
    pub shared_with_tenants: HashSet<TenantId>,
    pub assigned_reviewer_id: HashSet<UserId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub due_date: Option<DateTime>,
    pub version: u64,
}

impl Report {
    pub fn new(id: ReportId, author_id: UserId, owner_tenant: TenantId) -> ReportBuilder {
        ReportBuilder::new(id, author_id, owner_tenant)
    }

    pub fn into_parts(self) -> ReportParts {
        let Self {
            id,
            title,
            content,
            report_type,
            status,
            author_id,
            owner_tenant,
            shared_with_tenants,
            assigned_reviewer_id,
            created_at,
            updated_at,
            due_date,
            version,
        } = self;
        ReportParts {
            id,
            title,
            content,
            report_type,
            status,
            author_id,
            owner_tenant,
            shared_with_tenants,
            assigned_reviewer_id,
            created_at,
            updated_at,
            due_date,
            version,
        }
    }

    pub fn belongs_to(&self, tenant: &TenantId) -> bool {
        &self.owner_tenant == tenant
    }

    pub fn is_shared_with(&self, tenant: &TenantId) -> bool {
        self.shared_with_tenants.contains(tenant)
    }

    pub fn is_author(&self, user: &UserId) -> bool {
        &self.author_id == user
    }

    pub fn is_reviewer(&self, user: &UserId) -> bool {
        self.assigned_reviewer_id.contains(user)
    }

    // Geters
    pub fn id(&self) -> &ReportId {
        &self.id
    }

    pub fn title(&self) -> &Title {
        &self.title
    }

    pub fn content(&self) -> &ReportContent {
        &self.content
    }

    pub fn report_type(&self) -> &ReportType {
        &self.report_type
    }

    pub fn status(&self) -> &ReportStatus {
        &self.status
    }

    pub fn owner_tenant(&self) -> &TenantId {
        &self.owner_tenant
    }

    pub fn shared_with_tenants(&self) -> &HashSet<TenantId> {
        &self.shared_with_tenants
    }

    pub fn author_id(&self) -> &UserId {
        &self.author_id
    }

    pub fn assigned_reviewer_id(&self) -> &HashSet<UserId> {
        &self.assigned_reviewer_id
    }

    pub fn created_at(&self) -> &DateTime {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime {
        &self.updated_at
    }

    pub fn due_date(&self) -> &Option<DateTime> {
        &self.due_date
    }

    pub fn version(&self) -> &u64 {
        &self.version
    }
}

#[derive(Debug, Clone)]
pub struct ReportBuilder {
    id: ReportId,
    content: Option<ReportContent>,
    report_type: Option<ReportType>,
    author_id: UserId,
    owner_tenant: TenantId,
    shared_with_tenants: HashSet<TenantId>,
    status: Option<ReportStatus>,
    reviewer_id: HashSet<UserId>,
    created_at: Option<DateTime>,
    due: Option<DateTime>,
    version: u64,
}

impl ReportBuilder {
    pub fn new(id: ReportId, author_id: UserId, owner_tenant: TenantId) -> Self {
        Self {
            content: None,
            report_type: None,
            author_id,
            reviewer_id: HashSet::new(),
            created_at: None,
            due: None,
            id,
            status: None,
            version: 1,
            owner_tenant,
            shared_with_tenants: HashSet::new(),
        }
    }
    pub fn set_status(&mut self, status: ReportStatus) -> &mut Self {
        self.status = Some(status);
        self
    }
    pub fn set_created_at(&mut self, created_at: DateTime) -> &mut Self {
        self.created_at = Some(created_at);
        self
    }
    pub fn set_version(&mut self, version: u64) -> &mut Self {
        self.version = version;
        self
    }

    pub fn set_due(&mut self, due: DateTime) -> &mut Self {
        self.due = Some(due);
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

    pub fn add_shared_tenants(&mut self, tenant_ids: HashSet<TenantId>) -> &mut Self {
        self.shared_with_tenants = tenant_ids;
        self
    }
    pub fn add_shared_tenant(&mut self, tenant_id: TenantId) -> &mut Self {
        self.shared_with_tenants.insert(tenant_id);
        self
    }

    pub fn build(self, title: &str, updated_at: DateTime) -> DomainResult<Report> {
        Ok(Report {
            id: self.id,
            title: Title::new(title)?,
            content: self.content.unwrap_or_default(),
            report_type: self.report_type.unwrap_or(ReportType::default()),
            status: self.status.unwrap_or(ReportStatus::default()),
            author_id: self.author_id,
            assigned_reviewer_id: self.reviewer_id,
            created_at: self.created_at.unwrap_or(updated_at),
            updated_at,
            due_date: self.due,
            version: self.version,
            owner_tenant: self.owner_tenant,
            shared_with_tenants: self.shared_with_tenants,
        })
    }
}
impl Event for Report {
    fn get_type(&self) -> &str {
        "REPORT"
    }
}
