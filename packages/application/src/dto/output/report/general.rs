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
            id: value.id().clone().clone(),
            title: value.title().clone().clone(),
            content: GeneralReportContentOutput::from(value.content().clone()),
            report_type: value.report_type().clone().clone(),
            author_id: value.author_id().clone().clone(),
            created_at: value.created_at().clone().clone(),
            updated_at: value.updated_at().clone().clone(),
            due_date: value.due_date().clone().clone(),
            version: value.version().clone().clone(),
            owner_tenant: value.owner_tenant().clone().clone(),
            shared_with_tenants: value.shared_with_tenants().clone().clone(),
        }
    }
}

impl From<ReportContent> for GeneralReportContentOutput {
    fn from(value: ReportContent) -> Self {
        Self {
            body: value.body().clone().clone(),
            attachments: value.attachments().clone().clone(),
        }
    }
}
