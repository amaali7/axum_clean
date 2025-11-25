use chrono::{DateTime, Utc};

use crate::UserId;

#[derive(Debug, Clone)]
pub struct ReviewComment {
    reviewer_id: UserId,
    comment: String,
    created_at: DateTime<Utc>,
}

impl ReviewComment {
    pub fn new(reviewer_id: UserId, comment: &str, created_at: DateTime<Utc>) -> Self {
        Self {
            reviewer_id,
            comment: comment.trim().to_string(),
            created_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReportContent {
    body: String,
    attachments: Vec<String>, // URLs or paths to attachments
    review_comments: Vec<ReviewComment>,
    rejection_reason: Option<String>,
}

impl ReportContent {
    pub fn new(body: String) -> Self {
        Self {
            body: body.trim().to_string(),
            attachments: Vec::new(),
            review_comments: Vec::new(),
            rejection_reason: None,
        }
    }

    pub fn add_attachment(&mut self, attachment_url: String) {
        self.attachments.push(attachment_url);
    }

    pub fn add_review_comment(&mut self, reviewer_id: UserId, comment: String) {
        self.review_comments.push(ReviewComment {
            reviewer_id,
            comment: comment.trim().to_string(),
            created_at: Utc::now(),
        });
    }

    pub fn add_rejection_reason(&mut self, rejecter_id: UserId, reason: String) {
        self.rejection_reason = Some(reason.trim().to_string());
        self.add_review_comment(rejecter_id, format!("Rejected: {}", reason));
    }

    pub fn is_empty(&self) -> bool {
        self.body.trim().is_empty()
    }

    // Getters
    pub fn body(&self) -> &str {
        &self.body
    }
    pub fn attachments(&self) -> &[String] {
        &self.attachments
    }
    pub fn review_comments(&self) -> &[ReviewComment] {
        &self.review_comments
    }
}
