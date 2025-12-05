use std::collections::HashSet;

use domain::{
    events::DomainEventId,
    report::content::ReviewComment,
    value_objects::{Body, Comment, DateTime, Title, Url},
    Permission, Report, ReportContent, ReportId, ReportStatus, ReportType, UserId,
};

use crate::error::ApplicationError;

/// Auther User Report Output
pub struct AutherReportOutput {
    pub id: ReportId,
    pub title: Title,
    pub content: AutherReportContentOutput,
    pub report_type: ReportType,
    pub permissions: HashSet<Permission>,
    pub status: ReportStatus,
    pub author_id: UserId,
    pub assigned_reviewer_id: HashSet<UserId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub due_date: Option<DateTime>,
    pub version: u64,
    pub events: Vec<DomainEventId>,
}

pub struct AutherReportContentOutput {
    pub body: Body,
    pub attachments: Vec<Url>, // URLs or paths to attachments
    pub review_comments: Vec<AutherReviewCommentOutput>,
    pub rejection_reason: Option<Comment>,
}

pub struct AutherReviewCommentOutput {
    pub reviewer_id: UserId,
    pub comment: Comment,
    pub created_at: DateTime,
}

/// Mappers from Domain

impl From<Report> for AutherReportOutput {
    fn from(value: Report) -> Self {
        Self {
            id: value.id(),
            title: value.title(),
            content: AutherReportContentOutput::from(value.content()),
            report_type: value.report_type(),
            permissions: value.permissions(),
            status: value.status(),
            author_id: value.author_id(),
            assigned_reviewer_id: value.assigned_reviewer_id(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
            due_date: value.due_date(),
            version: value.version(),
            events: value.events(),
        }
    }
}

impl From<ReportContent> for AutherReportContentOutput {
    fn from(value: ReportContent) -> Self {
        Self {
            body: value.body(),
            attachments: value.attachments(),
            review_comments: value
                .review_comments()
                .into_iter()
                .map(|item| AutherReviewCommentOutput::from(item))
                .collect(),
            rejection_reason: value.rejection_reason(),
        }
    }
}

impl From<ReviewComment> for AutherReviewCommentOutput {
    fn from(value: ReviewComment) -> Self {
        Self {
            reviewer_id: value.reviewer_id(),
            comment: value.comment(),
            created_at: value.created_at(),
        }
    }
}
/// Mappers to Domain
impl TryFrom<AutherReportOutput> for Report {
    type Error = ApplicationError;

    fn try_from(value: AutherReportOutput) -> Result<Self, Self::Error> {
        let mut builder = Self::new(value.id, value.author_id);
        builder
            .set_report_type(value.report_type)
            .set_due(value.due_date.unwrap_or_default())
            .set_content(ReportContent::try_from(value.content)?);
        for event in value.events.into_iter() {
            builder.add_event(event);
        }
        for permission in value.permissions.into_iter() {
            builder.add_permission(permission);
        }
        for reviewer in value.assigned_reviewer_id.into_iter() {
            builder.add_reviewer(reviewer);
        }
        let report = builder.build(&value.title, value.created_at)?;
        Ok(report)
    }
}

impl TryFrom<AutherReportContentOutput> for ReportContent {
    type Error = ApplicationError;

    fn try_from(value: AutherReportContentOutput) -> Result<Self, Self::Error> {
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

impl From<AutherReviewCommentOutput> for ReviewComment {
    fn from(value: AutherReviewCommentOutput) -> Self {
        Self::new(value.reviewer_id, value.comment, value.created_at)
    }
}
