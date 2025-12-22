use domain::{error::ReportError, report::content::ReviewComment, DomainError, ReportContent};
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::{
            comment::SerializedComment, date_time::SerializedDateTime, SerializedBody,
            SerializedUrl,
        },
        SerializedUserId,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SerializedReviewComment {
    reviewer_id: SerializedUserId,
    comment: SerializedComment,
    created_at: SerializedDateTime,
}

impl SerializedReviewComment {
    pub fn new(
        reviewer_id: SerializedUserId,
        comment: SerializedComment,
        created_at: SerializedDateTime,
    ) -> Self {
        Self {
            reviewer_id,
            comment,
            created_at,
        }
    }
    pub fn reviewer_id(&self) -> SerializedUserId {
        self.reviewer_id.clone()
    }

    pub fn comment(&self) -> SerializedComment {
        self.comment.clone()
    }

    pub fn created_at(&self) -> SerializedDateTime {
        self.created_at.clone()
    }
}

impl TryFrom<ReviewComment> for SerializedReviewComment {
    type Error = InfrastructureError;

    fn try_from(value: ReviewComment) -> InfrastructureResult<Self> {
        Ok(Self::new(
            value.reviewer_id().into(),
            value.comment().try_into()?,
            value.created_at().try_into()?,
        ))
    }
}

impl TryFrom<SerializedReviewComment> for ReviewComment {
    type Error = InfrastructureError;

    fn try_from(value: SerializedReviewComment) -> InfrastructureResult<Self> {
        Ok(Self::new(
            value.reviewer_id().into(),
            value.comment().try_into()?,
            value.created_at().try_into()?,
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SerializedReportContent {
    body: SerializedBody,
    attachments: Vec<SerializedUrl>, // URLs or paths to attachments
    review_comments: Vec<SerializedReviewComment>,
    rejection_reason: Option<SerializedComment>,
}

impl SerializedReportContent {
    /// Creates a new [`SerializedReportContent`].
    pub fn new() -> SerializedReportContentBuilder {
        SerializedReportContentBuilder::new()
    }

    pub fn body(&self) -> SerializedBody {
        self.body.clone()
    }

    pub fn attachments(&self) -> Vec<SerializedUrl> {
        self.attachments.clone()
    }

    pub fn review_comments(&self) -> Vec<SerializedReviewComment> {
        self.review_comments.clone()
    }

    pub fn rejection_reason(&self) -> Option<SerializedComment> {
        self.rejection_reason.clone()
    }
}

#[derive(Debug, Clone)]
pub struct SerializedReportContentBuilder {
    body: Option<SerializedBody>,
    attachments: Vec<SerializedUrl>, // URLs or paths to attachments
    review_comments: Vec<SerializedReviewComment>,
    rejection_reason: Option<SerializedComment>,
}

impl SerializedReportContentBuilder {
    pub fn new() -> Self {
        Self {
            body: None,
            attachments: Vec::new(),
            review_comments: Vec::new(),
            rejection_reason: None,
        }
    }

    pub fn add_attachment(&mut self, attachment: SerializedUrl) -> &mut Self {
        self.attachments.push(attachment);
        self
    }

    pub fn add_review_comment(&mut self, review_comment: SerializedReviewComment) -> &mut Self {
        self.review_comments.push(review_comment);
        self
    }

    pub fn set_rejection_reason(&mut self, rejection_reason: SerializedComment) -> &mut Self {
        self.rejection_reason = Some(rejection_reason);
        self
    }

    pub fn set_body(&mut self, body: SerializedBody) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> InfrastructureResult<SerializedReportContent> {
        Ok(SerializedReportContent {
            body: self
                .body
                .ok_or(DomainError::ReportError(ReportError::BodyEmpty))?,
            attachments: self.attachments,
            review_comments: self.review_comments,
            rejection_reason: self.rejection_reason,
        })
    }
}

impl TryFrom<ReportContent> for SerializedReportContent {
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

impl TryFrom<SerializedReportContent> for ReportContent {
    type Error = InfrastructureError;

    fn try_from(value: SerializedReportContent) -> InfrastructureResult<Self> {
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
