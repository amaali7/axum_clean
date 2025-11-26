use crate::UserId;

#[derive(Debug, Clone)]
pub struct ReviewComment {
    reviewer_id: UserId,
    comment: String,
    created_at: String,
}

impl ReviewComment {
    pub fn new(reviewer_id: UserId, comment: &str, created_at: String) -> Self {
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
}
