use std::collections::HashSet;

use domain::{
    events::DomainEventId,
    report::content::ReviewComment,
    value_objects::{Body, Comment, DateTime, Title, Url},
    Permission, Report, ReportContent, ReportId, ReportStatus, ReportType, UserId,
};

use crate::error::ApplicationError;
/// Preivileg User Report Output
pub struct UpdateReportInput {
    pub id: ReportId,
    pub title: Title,
    pub content: UpdateReportContentInput,
    pub report_type: ReportType,
    pub permissions: HashSet<Permission>,
    pub status: ReportStatus,
    pub author_id: UserId,
    pub assigned_reviewer_id: HashSet<UserId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub due_date: Option<DateTime>,
    pub version: u64,
}

pub struct UpdateReportContentInput {
    pub body: Body,
    pub attachments: Vec<Url>, // URLs or paths to attachments
    pub review_comments: Vec<UpdateReviewCommentInput>,
    pub rejection_reason: Option<Comment>,
}

pub struct UpdateReviewCommentInput {
    pub reviewer_id: UserId,
    pub comment: Comment,
    pub created_at: DateTime,
}

/// Mapper from Domain

impl From<ReviewComment> for UpdateReviewCommentInput {
    fn from(value: ReviewComment) -> Self {
        Self {
            reviewer_id: value.reviewer_id(),
            comment: value.comment(),
            created_at: value.created_at(),
        }
    }
}

impl From<ReportContent> for UpdateReportContentInput {
    fn from(value: ReportContent) -> Self {
        Self {
            body: value.body(),
            attachments: value.attachments(),
            review_comments: value
                .review_comments()
                .into_iter()
                .map(|item| UpdateReviewCommentInput::from(item))
                .collect(),
            rejection_reason: value.rejection_reason(),
        }
    }
}

impl From<Report> for UpdateReportInput {
    fn from(value: Report) -> Self {
        Self {
            title: value.title(),
            content: UpdateReportContentInput::from(value.content()),
            report_type: value.report_type(),
            permissions: value.permissions(),
            status: value.status(),
            author_id: value.author_id(),
            assigned_reviewer_id: value.assigned_reviewer_id(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
            due_date: value.due_date(),
            version: value.version(),
            id: value.id(),
        }
    }
}

/// Mapper from DOT

impl TryFrom<UpdateReportInput> for Report {
    type Error = ApplicationError;

    fn try_from(value: UpdateReportInput) -> Result<Self, Self::Error> {
        let mut builder = Self::new(value.id, value.author_id);
        builder
            .set_report_type(value.report_type)
            .set_due(value.due_date.unwrap_or_default())
            .set_status(value.status)
            .set_version(value.version)
            .set_created_at(value.created_at)
            .set_content(ReportContent::try_from(value.content)?);
        for permission in value.permissions.into_iter() {
            builder.add_permission(permission);
        }
        for reviewer in value.assigned_reviewer_id.into_iter() {
            builder.add_reviewer(reviewer);
        }
        let report = builder.build(&value.title, value.updated_at)?;
        Ok(report)
    }
}

impl TryFrom<UpdateReportContentInput> for ReportContent {
    type Error = ApplicationError;

    fn try_from(value: UpdateReportContentInput) -> Result<Self, Self::Error> {
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

impl From<UpdateReviewCommentInput> for ReviewComment {
    fn from(value: UpdateReviewCommentInput) -> Self {
        Self::new(value.reviewer_id, value.comment, value.created_at)
    }
}
