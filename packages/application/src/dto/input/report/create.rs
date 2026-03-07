use std::collections::HashSet;

use domain::{
    report::content::ReviewComment,
    value_objects::{Body, Comment, DateTime, Title, Url},
    Report, ReportContent, ReportId, ReportStatus, ReportType, TenantId, UserId,
};

use crate::error::AppError;
/// Preivileg User Report Output
pub struct CreateReportInput {
    pub id: ReportId,
    pub title: Title,
    pub content: CreateReportContentInput,
    pub report_type: ReportType,
    pub status: ReportStatus,
    pub author_id: UserId,
    pub owner_tenant: TenantId,
    pub shared_with_tenants: HashSet<TenantId>,
    pub assigned_reviewer_id: HashSet<UserId>,
    pub due_date: Option<DateTime>,
    pub version: u64,
}

pub struct CreateReportContentInput {
    pub body: Body,
    pub attachments: HashSet<Url>, // URLs or paths to attachments
    pub review_comments: HashSet<CreateReviewCommentInput>,
    pub rejection_reason: Option<Comment>,
}

#[derive(Debug, Eq, PartialEq, Default, Hash, Clone)]
pub struct CreateReviewCommentInput {
    pub reviewer_id: UserId,
    pub comment: Comment,
}

/// Mapper From DOT

impl TryFrom<CreateReportInput> for Report {
    type Error = AppError;

    fn try_from(value: CreateReportInput) -> Result<Self, Self::Error> {
        let mut builder = Self::new(value.id, value.author_id, value.owner_tenant);
        builder
            .set_report_type(value.report_type)
            .set_due(value.due_date.unwrap_or_default())
            .set_status(value.status)
            .set_version(value.version)
            .add_shared_tenants(value.shared_with_tenants)
            .set_content(ReportContent::try_from(value.content)?);

        for reviewer in value.assigned_reviewer_id.into_iter() {
            builder.add_reviewer(reviewer);
        }
        let report = builder.build(&value.title, DateTime::new(0))?;
        Ok(report)
    }
}

impl TryFrom<CreateReportContentInput> for ReportContent {
    type Error = AppError;

    fn try_from(value: CreateReportContentInput) -> Result<Self, Self::Error> {
        let mut builder = Self::new();
        builder
            .set_body(value.body)
            .set_rejection_reason(value.rejection_reason.unwrap_or_default());
        for attachment in value.attachments.into_iter() {
            builder.add_attachment(attachment);
        }
        for review_comment in value.review_comments.into_iter() {
            builder.add_review_comment(ReviewComment::from(review_comment));
        }
        let report_content = builder.build()?;
        Ok(report_content)
    }
}

impl From<CreateReviewCommentInput> for ReviewComment {
    fn from(value: CreateReviewCommentInput) -> Self {
        Self::new(value.reviewer_id, value.comment, DateTime::new(0))
    }
}
