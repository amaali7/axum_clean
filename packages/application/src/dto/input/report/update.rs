use std::collections::HashSet;

use domain::{
    report::content::ReviewComment,
    value_objects::{Body, Comment, DateTime, Title, Url},
    Report, ReportContent, ReportId, ReportStatus, ReportType, TenantId, UserId,
};

use crate::error::AppError;
/// Preivileg User Report Output
#[derive(Debug, Default, Clone)]
pub struct UpdateReportInput {
    pub id: ReportId,
    pub title: Option<Title>,
    pub content: UpdateReportContentInput,
    pub report_type: Option<ReportType>,
    pub status: Option<ReportStatus>,
    pub author_id: Option<UserId>,
    pub owner_tenant: Option<TenantId>,
    pub shared_with_tenants: HashSet<TenantId>,
    pub assigned_reviewer_id: HashSet<UserId>,
    pub due_date: Option<DateTime>,
    pub version: Option<u64>,
}

#[derive(Debug, Default, Clone)]
pub struct UpdateReportContentInput {
    pub body: Option<Body>,
    pub attachments: HashSet<Url>, // URLs or paths to attachments
    pub review_comments: HashSet<UpdateReviewCommentInput>,
    pub rejection_reason: Option<Comment>,
}

#[derive(Debug, Eq, PartialEq, Default, Hash, Clone)]
pub struct UpdateReviewCommentInput {
    pub reviewer_id: Option<UserId>,
    pub comment: Option<Comment>,
}

/// Mapper from DOT

impl TryFrom<UpdateReportInput> for Report {
    type Error = AppError;

    fn try_from(value: UpdateReportInput) -> Result<Self, Self::Error> {
        let mut builder = Self::new(
            value.id,
            value.author_id.unwrap_or_default(),
            value.owner_tenant.unwrap_or_default(),
        );
        builder
            .set_report_type(value.report_type.unwrap_or_default())
            .set_due(value.due_date.unwrap_or_default())
            .set_status(value.status.unwrap_or_default())
            .set_version(value.version.unwrap_or_default())
            .add_shared_tenants(value.shared_with_tenants)
            .set_created_at(DateTime::default())
            .set_content(ReportContent::try_from(value.content)?);
        for reviewer in value.assigned_reviewer_id.into_iter() {
            builder.add_reviewer(reviewer);
        }
        let report = builder.build(&value.title.unwrap_or_default(), DateTime::default())?;
        Ok(report)
    }
}

impl TryFrom<UpdateReportContentInput> for ReportContent {
    type Error = AppError;

    fn try_from(value: UpdateReportContentInput) -> Result<Self, Self::Error> {
        let mut builder = Self::new();
        builder
            .set_body(value.body.unwrap_or_default())
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

impl From<UpdateReviewCommentInput> for ReviewComment {
    fn from(value: UpdateReviewCommentInput) -> Self {
        Self::new(
            value.reviewer_id.unwrap_or_default(),
            value.comment.unwrap_or_default(),
            DateTime::default(),
        )
    }
}
