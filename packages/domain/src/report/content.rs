use crate::{
    error::DomainResult,
    value_objects::{Body, Comment, DateTime, Url},
    DomainError, UserId,
};

#[derive(Debug, Clone, Default)]
pub struct ReviewComment {
    reviewer_id: UserId,
    comment: Comment,
    created_at: DateTime,
}

impl ReviewComment {
    pub fn new(reviewer_id: UserId, comment: Comment, created_at: DateTime) -> Self {
        Self {
            reviewer_id,
            comment,
            created_at,
        }
    }
    pub fn reviewer_id(&self) -> UserId {
        self.reviewer_id.clone()
    }

    pub fn comment(&self) -> Comment {
        self.comment.clone()
    }

    pub fn created_at(&self) -> DateTime {
        self.created_at.clone()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReportContent {
    body: Body,
    attachments: Vec<Url>, // URLs or paths to attachments
    review_comments: Vec<ReviewComment>,
    rejection_reason: Option<Comment>,
}

impl ReportContent {
    /// Creates a new [`ReportContent`].
    pub fn new() -> ReportContentBuilder {
        ReportContentBuilder::new()
    }

    pub fn body(&self) -> Body {
        self.body.clone()
    }

    pub fn attachments(&self) -> Vec<Url> {
        self.attachments.clone()
    }

    pub fn review_comments(&self) -> Vec<ReviewComment> {
        self.review_comments.clone()
    }

    pub fn rejection_reason(&self) -> Option<Comment> {
        self.rejection_reason.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ReportContentBuilder {
    body: Option<Body>,
    attachments: Vec<Url>, // URLs or paths to attachments
    review_comments: Vec<ReviewComment>,
    rejection_reason: Option<Comment>,
}

impl Default for ReportContentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportContentBuilder {
    pub fn new() -> Self {
        Self {
            body: None,
            attachments: Vec::new(),
            review_comments: Vec::new(),
            rejection_reason: None,
        }
    }

    pub fn add_attachment(&mut self, attachment: Url) -> &mut Self {
        self.attachments.push(attachment);
        self
    }

    pub fn add_review_comment(&mut self, review_comment: ReviewComment) -> &mut Self {
        self.review_comments.push(review_comment);
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
