use domain::{error::ReportError, report::content::ReviewComment, DomainError, ReportContent};
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::{
            comment::InfrastructureComment, date_time::InfrastructureDateTime, InfrastructureBody,
            InfrastructureUrl,
        },
        InfrastructureUserId,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfrastructureReviewComment {
    reviewer_id: InfrastructureUserId,
    comment: InfrastructureComment,
    created_at: InfrastructureDateTime,
}

impl InfrastructureReviewComment {
    pub fn new(
        reviewer_id: InfrastructureUserId,
        comment: InfrastructureComment,
        created_at: InfrastructureDateTime,
    ) -> Self {
        Self {
            reviewer_id,
            comment,
            created_at,
        }
    }
    pub fn reviewer_id(&self) -> InfrastructureUserId {
        self.reviewer_id.clone()
    }

    pub fn comment(&self) -> InfrastructureComment {
        self.comment.clone()
    }

    pub fn created_at(&self) -> InfrastructureDateTime {
        self.created_at.clone()
    }
}

impl TryFrom<ReviewComment> for InfrastructureReviewComment {
    type Error = InfrastructureError;

    fn try_from(value: ReviewComment) -> InfrastructureResult<Self> {
        Ok(Self::new(
            value.reviewer_id().into(),
            value.comment().try_into()?,
            value.created_at().try_into()?,
        ))
    }
}

impl TryFrom<InfrastructureReviewComment> for ReviewComment {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureReviewComment) -> InfrastructureResult<Self> {
        Ok(Self::new(
            value.reviewer_id().into(),
            value.comment().try_into()?,
            value.created_at().try_into()?,
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfrastructureReportContent {
    body: InfrastructureBody,
    attachments: Vec<InfrastructureUrl>, // URLs or paths to attachments
    review_comments: Vec<InfrastructureReviewComment>,
    rejection_reason: Option<InfrastructureComment>,
}

impl InfrastructureReportContent {
    /// Creates a new [`InfrastructureReportContent`].
    pub fn new() -> InfrastructureReportContentBuilder {
        InfrastructureReportContentBuilder::new()
    }

    pub fn body(&self) -> InfrastructureBody {
        self.body.clone()
    }

    pub fn attachments(&self) -> Vec<InfrastructureUrl> {
        self.attachments.clone()
    }

    pub fn review_comments(&self) -> Vec<InfrastructureReviewComment> {
        self.review_comments.clone()
    }

    pub fn rejection_reason(&self) -> Option<InfrastructureComment> {
        self.rejection_reason.clone()
    }
}

#[derive(Debug, Clone)]
pub struct InfrastructureReportContentBuilder {
    body: Option<InfrastructureBody>,
    attachments: Vec<InfrastructureUrl>, // URLs or paths to attachments
    review_comments: Vec<InfrastructureReviewComment>,
    rejection_reason: Option<InfrastructureComment>,
}

impl InfrastructureReportContentBuilder {
    pub fn new() -> Self {
        Self {
            body: None,
            attachments: Vec::new(),
            review_comments: Vec::new(),
            rejection_reason: None,
        }
    }

    pub fn add_attachment(&mut self, attachment: InfrastructureUrl) -> &mut Self {
        self.attachments.push(attachment);
        self
    }

    pub fn add_review_comment(&mut self, review_comment: InfrastructureReviewComment) -> &mut Self {
        self.review_comments.push(review_comment);
        self
    }

    pub fn set_rejection_reason(&mut self, rejection_reason: InfrastructureComment) -> &mut Self {
        self.rejection_reason = Some(rejection_reason);
        self
    }

    pub fn set_body(&mut self, body: InfrastructureBody) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> InfrastructureResult<InfrastructureReportContent> {
        Ok(InfrastructureReportContent {
            body: self
                .body
                .ok_or(DomainError::ReportError(ReportError::BodyEmpty))?,
            attachments: self.attachments,
            review_comments: self.review_comments,
            rejection_reason: self.rejection_reason,
        })
    }
}

impl TryFrom<ReportContent> for InfrastructureReportContent {
    type Error = InfrastructureError;

    fn try_from(value: ReportContent) -> InfrastructureResult<Self> {
        let mut report_content_builder = Self::new();
        report_content_builder.set_body(value.body().try_into()?);
        match value.rejection_reason() {
            Some(rejection_reason) => {
                report_content_builder.set_rejection_reason(rejection_reason.try_into()?);
            }
            None => (),
        }
        for attachment in value.attachments().into_iter() {
            report_content_builder.add_attachment(attachment.try_into()?);
        }
        for review_comment in value.review_comments().into_iter() {
            report_content_builder.add_review_comment(review_comment.try_into()?);
        }
        report_content_builder.build()
    }
}

impl TryFrom<InfrastructureReportContent> for ReportContent {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureReportContent) -> InfrastructureResult<Self> {
        let mut report_content_builder = Self::new();
        report_content_builder.set_body(value.body().try_into()?);
        match value.rejection_reason() {
            Some(rejection_reason) => {
                report_content_builder.set_rejection_reason(rejection_reason.try_into()?);
            }
            None => (),
        }
        for attachment in value.attachments().into_iter() {
            report_content_builder.add_attachment(attachment.try_into()?);
        }
        for review_comment in value.review_comments().into_iter() {
            report_content_builder.add_review_comment(review_comment.try_into()?);
        }
        report_content_builder
            .build()
            .map_err(|err| InfrastructureError::Domain(err))
    }
}
