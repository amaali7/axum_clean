use std::collections::HashSet;

use domain::{
    report::content::ReviewComment,
    value_objects::{Body, Comment, DateTime, Title, Url},
    Report, ReportContent, ReportId, ReportStatus, ReportType, TenantId, UserId,
};
/// Reviewer User Report Output
pub struct ReviewerReportOutput {
    pub id: ReportId,
    pub title: Title,
    pub content: ReviewerReportContentOutput,
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

pub struct ReviewerReportContentOutput {
    pub body: Body,
    pub attachments: HashSet<Url>, // URLs or paths to attachments
    pub review_comments: HashSet<ReviewerReviewCommentOutput>,
    pub rejection_reason: Option<Comment>,
}

#[derive(Debug, Eq, PartialEq, Default, Hash, Clone)]
pub struct ReviewerReviewCommentOutput {
    pub reviewer_id: UserId,
    pub comment: Comment,
    pub created_at: DateTime,
}

impl From<Report> for ReviewerReportOutput {
    fn from(value: Report) -> Self {
        Self {
            id: value.id().clone().clone(),
            title: value.title().clone().clone(),
            content: ReviewerReportContentOutput::from(value.content().clone()),
            report_type: value.report_type().clone().clone(),
            status: value.status().clone().clone(),
            author_id: value.author_id().clone().clone(),
            assigned_reviewer_id: value.assigned_reviewer_id().clone().clone(),
            created_at: value.created_at().clone().clone(),
            updated_at: value.updated_at().clone().clone(),
            due_date: value.due_date().clone().clone(),
            version: value.version().clone().clone(),
            owner_tenant: value.owner_tenant().clone().clone(),
            shared_with_tenants: value.shared_with_tenants().clone().clone(),
        }
    }
}

impl From<ReportContent> for ReviewerReportContentOutput {
    fn from(value: ReportContent) -> Self {
        Self {
            body: value.body().clone().clone(),
            attachments: value.attachments().clone().clone(),
            review_comments: value
                .review_comments()
                .into_iter()
                .map(|item| ReviewerReviewCommentOutput::from(item.clone()))
                .collect::<HashSet<_>>()
                .clone(),
            rejection_reason: value.rejection_reason().clone().clone(),
        }
    }
}

impl From<ReviewComment> for ReviewerReviewCommentOutput {
    fn from(value: ReviewComment) -> Self {
        Self {
            reviewer_id: value.reviewer_id().clone().clone(),
            comment: value.comment().clone().clone(),
            created_at: value.created_at().clone().clone(),
        }
    }
}
