use std::collections::HashSet;

use domain::{
    value_objects::{Body, Title, Url},
    DateTime, Report, ReportContent, ReportId, ReportType, TenantId, UserId,
};

/// General Report Output
pub struct GeneralReportOutput {
    pub id: ReportId,
    pub title: Title,
    pub content: GeneralReportContentOutput,
    pub report_type: ReportType,
    pub author_id: UserId,
    pub owner_tenant: TenantId,
    pub shared_with_tenants: HashSet<TenantId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub due_date: Option<DateTime>,
    pub version: u64,
}
pub struct GeneralReportContentOutput {
    pub body: Body,
    pub attachments: HashSet<Url>, // URLs or paths to attachments
}

/// Mappers from Domain

impl From<Report> for GeneralReportOutput {
    fn from(value: Report) -> Self {
        Self {
            id: value.id(),
            title: value.title(),
            content: GeneralReportContentOutput::from(value.content()),
            report_type: value.report_type(),
            author_id: value.author_id(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
            due_date: value.due_date(),
            version: value.version(),
            owner_tenant: value.owner_tenant(),
            shared_with_tenants: value.shared_with_tenants(),
        }
    }
}

impl From<ReportContent> for GeneralReportContentOutput {
    fn from(value: ReportContent) -> Self {
        Self {
            body: value.body(),
            attachments: value.attachments(),
        }
    }
}
