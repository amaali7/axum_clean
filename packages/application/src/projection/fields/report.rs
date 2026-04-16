use domain::traits::field::Field;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepotField {
    Id,
    Title,
    Content(ReportContentField),
    ReportType(ReportTypeField),
    Status,
    AuthorId,
    OwnerTenant,
    SharedWithTenants,
    AssignedReviewerId,
    CreatedAt,
    UpdatedAt,
    DueDate,
    Version,
}

impl Field for RepotField {
    fn name(&self) -> &'static str {
        match self {
            RepotField::Id => "id",
            RepotField::Title => "title",
            RepotField::Content(_) => "content",
            RepotField::ReportType(_) => "report_type", // FIXME: Must have tenant field
            RepotField::Status => "status",
            RepotField::AuthorId => "author_id",
            RepotField::OwnerTenant => "owner_tenant",
            RepotField::SharedWithTenants => "shared_with_tenants", // FIXME: array
            RepotField::AssignedReviewerId => "assigned_reviewer_id",
            RepotField::CreatedAt => "created_at",
            RepotField::UpdatedAt => "updated_at",
            RepotField::DueDate => "due_date",
            RepotField::Version => "version",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReviewCommentField {
    ReviewerId,
    Comment,
    CreatedAt,
}

impl Field for ReviewCommentField {
    fn name(&self) -> &'static str {
        match self {
            ReviewCommentField::ReviewerId => "reviewer_id",
            ReviewCommentField::Comment => "comment",
            ReviewCommentField::CreatedAt => "created_at",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportContentField {
    Body,
    Attachments, // URLs or paths to attachments
    ReviewComments(ReviewCommentField),
    RejectionReason,
}

impl Field for ReportContentField {
    fn name(&self) -> &'static str {
        match self {
            ReportContentField::Body => "body",
            ReportContentField::Attachments => "attachments",
            ReportContentField::ReviewComments(_) => "review_comment",
            ReportContentField::RejectionReason => "rejection_reason",
        }
    }
}
// FIXME: Add field for tenant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportTypeField {
    Name,
    Id,
    Description,
    CreatedAt,
}

impl Field for ReportTypeField {
    fn name(&self) -> &'static str {
        match self {
            ReportTypeField::Name => "name",
            ReportTypeField::Id => "id",
            ReportTypeField::Description => "description",
            ReportTypeField::CreatedAt => "created_at",
        }
    }
}
