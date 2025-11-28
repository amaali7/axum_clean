use crate::{
    value_objects::{Body, Comment, DateTime, Url},
    UserId,
};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ReportContent {
    body: Body,
    attachments: Vec<Url>, // URLs or paths to attachments
    review_comments: Vec<ReviewComment>,
    rejection_reason: Option<Comment>,
}

impl ReportContent {
    /// Creates a new [`ReportContent`].
    pub fn new(body: Body) -> Self {
        Self {
            body,
            attachments: Vec::new(),
            review_comments: Vec::new(),
            rejection_reason: None,
        }
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
