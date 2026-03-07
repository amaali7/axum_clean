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
            id: value.id(),
            title: value.title(),
            content: ReviewerReportContentOutput::from(value.content()),
            report_type: value.report_type(),
            status: value.status(),
            author_id: value.author_id(),
            assigned_reviewer_id: value.assigned_reviewer_id(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
            due_date: value.due_date(),
            version: value.version(),
            owner_tenant: value.owner_tenant(),
            shared_with_tenants: value.shared_with_tenants(),
        }
    }
}

impl From<ReportContent> for ReviewerReportContentOutput {
    fn from(value: ReportContent) -> Self {
        Self {
            body: value.body(),
            attachments: value.attachments(),
            review_comments: value
                .review_comments()
                .into_iter()
                .map(|item| ReviewerReviewCommentOutput::from(item))
                .collect(),
            rejection_reason: value.rejection_reason(),
        }
    }
}

impl From<ReviewComment> for ReviewerReviewCommentOutput {
    fn from(value: ReviewComment) -> Self {
        Self {
            reviewer_id: value.reviewer_id(),
            comment: value.comment(),
            created_at: value.created_at(),
        }
    }
}
