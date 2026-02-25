use std::collections::HashSet;

use application::dto::report_dto::input::{
    UpdateReportContentInput, UpdateReportInput, UpdateReviewCommentInput,
};

use crate::{
    common_objects::{
        report::{
            report_type::InterfaceReportType, status::InterfaceReportStatus, InterfaceReportId,
        },
        role::permissions::InterfacePermission,
        user::InterfaceUserId,
    },
    error::{InterfaceError, InterfaceResult},
    value_objects::{
        InterfaceBody, InterfaceComment, InterfaceDateTime, InterfaceTitle, InterfaceUrl,
    },
};
/// Preivileg User Report Response
pub struct UpdateReportRequest {
    pub id: InterfaceReportId,
    pub title: Option<InterfaceTitle>,
    pub content: UpdateReportContentRequest,
    pub report_type: Option<InterfaceReportType>,
    pub permissions: HashSet<InterfacePermission>,
    pub status: Option<InterfaceReportStatus>,
    pub author_id: Option<InterfaceUserId>,
    pub assigned_reviewer_id: HashSet<InterfaceUserId>,
    pub due_date: Option<InterfaceDateTime>,
    pub version: Option<u64>,
}

pub struct UpdateReportContentRequest {
    pub body: Option<InterfaceBody>,
    pub attachments: Vec<InterfaceUrl>, // URLs or paths to attachments
    pub review_comments: Vec<UpdateReviewCommentRequest>,
    pub rejection_reason: Option<InterfaceComment>,
}

pub struct UpdateReviewCommentRequest {
    pub reviewer_id: Option<InterfaceUserId>,
    pub comment: Option<InterfaceComment>,
}

/// Mapper from Domain

impl TryFrom<UpdateReviewCommentRequest> for UpdateReviewCommentInput {
    type Error = InterfaceError;
    fn try_from(value: UpdateReviewCommentRequest) -> InterfaceResult<Self> {
        Ok(Self {
            reviewer_id: value.reviewer_id.map(|a| a.into()),
            comment: value.comment.map(|x| x.try_into()).transpose()?,
        })
    }
}

impl TryFrom<UpdateReportContentRequest> for UpdateReportContentInput {
    type Error = InterfaceError;
    fn try_from(value: UpdateReportContentRequest) -> InterfaceResult<Self> {
        Ok(Self {
            body: value.body.map(|x| x.try_into()).transpose()?,
            attachments: {
                let mut attachments = Vec::new();
                for url in value.attachments {
                    attachments.push(url.try_into()?);
                }
                attachments
            },
            review_comments: {
                let mut review_comments = Vec::new();
                for comment in value.review_comments {
                    review_comments.push(comment.try_into()?);
                }
                review_comments
            },
            rejection_reason: value.rejection_reason.map(|a| a.try_into()).transpose()?,
        })
    }
}

impl TryFrom<UpdateReportRequest> for UpdateReportInput {
    type Error = InterfaceError;
    fn try_from(value: UpdateReportRequest) -> InterfaceResult<Self> {
        Ok(Self {
            title: value.title.map(|x| x.try_into()).transpose()?,
            content: UpdateReportContentInput::try_from(value.content)?,
            report_type: value.report_type.map(|x| x.into()),
            permissions: value
                .permissions
                .into_iter()
                .map(|permission| permission.into())
                .collect(),
            status: value.status.map(|x| x.into()),
            author_id: value.author_id.map(|x| x.into()),
            assigned_reviewer_id: {
                let mut assigned_reviewer_id = HashSet::new();
                for id in value.assigned_reviewer_id {
                    assigned_reviewer_id.insert(id.into());
                }
                assigned_reviewer_id
            },
            due_date: value.due_date.map(|x| x.try_into()).transpose()?,
            version: value.version,
            id: value.id.into(),
        })
    }
}
