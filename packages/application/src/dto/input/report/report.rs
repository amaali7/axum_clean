use std::collections::{HashSet, VecDeque};

use domain::{events::DomainEvent, value_objects::Title, Permission, ReportStatus, UserId};
/// Preivileg User Report Output
pub struct ReportInput {
    title: Title,
    content: ReportContentInput,
    report_type: ReportTypeInput,
    permissions: HashSet<Permission>,
    status: ReportStatus,
    author_id: UserId,
    assigned_reviewer_id: Option<HashSet<UserId>>,
    created_at: String,
    updated_at: String,
    due_date: Option<String>,
    version: u64,
    events: VecDeque<DomainEvent>,
}

pub struct ReportContentInput {
    body: String,
    attachments: Vec<String>, // URLs or paths to attachments
    review_comments: Vec<ReviewCommentInput>,
    rejection_reason: Option<String>,
}

pub struct ReviewCommentInput {
    reviewer_id: UserId,
    comment: String,
    created_at: String,
}

pub enum ReportTypeInput {
    Financial,
    Technical,
    Progress,
    Incident,
    Audit,
    Other,
}
