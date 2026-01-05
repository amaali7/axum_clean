pub mod content;
pub use content::{ReportContentField, ReviewCommentFiled};

use crate::surreal::sql::schema::PathSegment;

use super::{FieldPath, QueryField};

#[derive(Debug, Clone)]
pub enum ReportField {
    Id,
    Title,
    Content,
    ReportType,
    Permissions,
    Status,
    AuthorId,
    AssignedReviewerId,
    CreatedAt,
    UpdatedAt,
    DueDate,
    Version,
    Events,
}

impl QueryField for ReportField {
    fn path(&self) -> FieldPath {
        vec![PathSegment::Field(
            match self {
                Self::Id => "id",
                Self::Title => "title",
                Self::Content => "content",
                Self::ReportType => "report_type",
                Self::Permissions => "permissions",
                Self::Status => "status",
                Self::AuthorId => "author_id",
                Self::AssignedReviewerId => "assign_reviewer_id",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "updated_at",
                Self::DueDate => "due_date",
                Self::Version => "version",
                Self::Events => "events",
            }
            .into(),
        )]
    }

    fn table_ref(&self) -> &str {
        "report"
    }
}

/* =========================
Typed traversal helpers
========================= */

/// user.profile
pub struct ContentPath {
    base: FieldPath,
}

impl ReportField {
    pub fn content(self) -> ContentPath {
        ContentPath { base: self.path() }
    }
}
