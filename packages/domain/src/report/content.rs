use std::collections::HashSet;

use crate::{
    error::DomainResult,
    value_objects::{Body, Comment, DateTime, Url},
    DomainError, UserId,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct ReviewComment {
    reviewer_id: UserId,
    comment: Comment,
    created_at: DateTime,
}

#[derive(Debug, Default)]
pub struct ReviewCommentParts {
    pub reviewer_id: UserId,
    pub comment: Comment,
    pub created_at: DateTime,
}

impl ReviewComment {
    pub fn new(reviewer_id: UserId, comment: Comment, created_at: DateTime) -> Self {
        Self {
            reviewer_id,
            comment,
            created_at,
        }
    }

    pub fn into_parts(self) -> ReviewCommentParts {
        let Self {
            reviewer_id,
            comment,
            created_at,
        } = self;
        ReviewCommentParts {
            reviewer_id,
            comment,
            created_at,
        }
    }
    pub fn reviewer_id(&self) -> &UserId {
        &self.reviewer_id
    }

    pub fn comment(&self) -> &Comment {
        &self.comment
    }

    pub fn created_at(&self) -> &DateTime {
        &self.created_at
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReportContent {
    body: Body,
    attachments: HashSet<Url>, // URLs or paths to attachments
    review_comments: HashSet<ReviewComment>,
    rejection_reason: Option<Comment>,
}
#[derive(Debug)]
pub struct ReportContentParts {
    pub body: Body,
    pub attachments: HashSet<Url>, // URLs or paths to attachments
    pub review_comments: HashSet<ReviewComment>,
    pub rejection_reason: Option<Comment>,
}

impl ReportContent {
    /// Creates a new [`ReportContent`].
    pub fn new() -> ReportContentBuilder {
        ReportContentBuilder::new()
    }

    pub fn into_parts(self) -> ReportContentParts {
        let Self {
            body,
            attachments,
            review_comments,
            rejection_reason,
        } = self;
        ReportContentParts {
            body,
            attachments,
            review_comments,
            rejection_reason,
        }
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn attachments(&self) -> &HashSet<Url> {
        &self.attachments
    }

    pub fn review_comments(&self) -> &HashSet<ReviewComment> {
        &self.review_comments
    }

    pub fn rejection_reason(&self) -> &Option<Comment> {
        &self.rejection_reason
    }
}

#[derive(Debug, Clone)]
pub struct ReportContentBuilder {
    body: Option<Body>,
    attachments: HashSet<Url>, // URLs or paths to attachments
    review_comments: HashSet<ReviewComment>,
    rejection_reason: Option<Comment>,
}

impl ReportContentBuilder {
    pub fn new() -> Self {
        Self {
            body: None,
            attachments: HashSet::new(),
            review_comments: HashSet::new(),
            rejection_reason: None,
        }
    }

    pub fn add_attachment(&mut self, attachment: Url) -> &mut Self {
        self.attachments.insert(attachment);
        self
    }

    pub fn add_review_comment(&mut self, review_comment: ReviewComment) -> &mut Self {
        self.review_comments.insert(review_comment);
        self
    }

    pub fn set_rejection_reason(&mut self, rejection_reason: Comment) -> &mut Self {
        self.rejection_reason = Some(rejection_reason);
        self
    }

    pub fn set_body(&mut self, body: Body) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> DomainResult<ReportContent> {
        Ok(ReportContent {
            body: self.body.ok_or(DomainError::ReportError(
                crate::error::ReportError::BodyEmpty,
            ))?,
            attachments: self.attachments,
            review_comments: self.review_comments,
            rejection_reason: self.rejection_reason,
        })
    }
}
