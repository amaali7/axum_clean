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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReviewCommentField {
    ReviewerId,
    Comment,
    CreatedAt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportContentField {
    Body,
    Attachments, // URLs or paths to attachments
    ReviewComments(ReviewCommentField),
    RejectionReason,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportTypeField {
    Name,
    Id,
    Description,
    CreatedAt,
}
