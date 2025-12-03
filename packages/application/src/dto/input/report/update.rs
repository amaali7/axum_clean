use std::collections::HashSet;

use domain::{
    events::DomainEventId,
    report::content::ReviewComment,
    value_objects::{Body, Comment, DateTime, Title, Url},
    Permission, Report, ReportContent, ReportId, ReportStatus, ReportType, UserId,
};
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
    pub events: Vec<DomainEventId>,
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
            events: value.events(),
            id: value.id(),
        }
    }
}
