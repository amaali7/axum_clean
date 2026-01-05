use crate::surreal::sql::schema::{FieldPath, PathSegment, QueryField};

#[derive(Debug, Clone)]
pub enum ReportContentField {
    Body,
    Attachments,
    Reviewcomments,
    RejectionReason,
}

impl QueryField for ReportContentField {
    fn path(&self) -> FieldPath {
        vec![PathSegment::Field(
            match self {
                Self::Body => "body",
                Self::Attachments => "attachments",
                Self::Reviewcomments => "review_comments",
                Self::RejectionReason => "rejection_reason",
            }
            .into(),
        )]
    }

    fn table_ref(&self) -> &str {
        "report"
    }
}

#[derive(Debug, Clone)]
pub enum ReviewCommentField {
    ReviewerId,
    Comment,
    CreatedAt,
}

impl QueryField for ReviewCommentField {
    fn path(&self) -> FieldPath {
        vec![PathSegment::Field(
            match self {
                Self::ReviewerId => "reviewer_id",
                Self::Comment => "comment",
                Self::CreatedAt => "created_at",
            }
            .into(),
        )]
    }

    fn table_ref(&self) -> &str {
        "report"
    }
}

pub struct ContentPath {
    base: FieldPath,
}

pub struct ReviewCommentListPath {
    base: FieldPath,
}

pub struct ReviewCommentItemPath {
    base: FieldPath,
}

impl ReportField {
    pub fn content(self) -> ContentPath {
        ContentPath { base: self.path() }
    }
}

impl ContentPath {
    pub fn review_comments(self) -> ReviewCommentListPath {
        let mut base = self.base;
        base.push(PathSegment::Field("review_comments".into()));
        ReviewCommentListPath { base }
    }
}

impl ReviewCommentListPath {
    pub fn any(self) -> ReviewCommentItemPath {
        let mut base = self.base;
        base.push(PathSegment::All);
        ReviewCommentItemPath { base }
    }
}

impl ReviewCommentItemPath {
    pub fn field(self, field: ReviewCommentField) -> FieldPath {
        let mut base = self.base;
        base.extend(field.path());
        base
    }
}
