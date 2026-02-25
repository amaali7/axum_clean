use std::{any::Any, collections::HashSet};

use application::dto::report_dto::output::{
    AutherReportContentOutput, AutherReportOutput, AutherReviewCommentOutput,
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

use super::reviewer::{self, ReviewerReviewCommentResponse};

/// Auther User Report Response
pub struct AutherReportResponse {
    pub id: InterfaceReportId,
    pub title: InterfaceTitle,
    pub content: AutherReportContentResponse,
    pub report_type: InterfaceReportType,
    pub permissions: HashSet<InterfacePermission>,
    pub status: InterfaceReportStatus,
    pub author_id: InterfaceUserId,
    pub assigned_reviewer_id: HashSet<InterfaceUserId>,
    pub created_at: InterfaceDateTime,
    pub updated_at: InterfaceDateTime,
    pub due_date: Option<InterfaceDateTime>,
    pub version: u64,
}

pub struct AutherReportContentResponse {
    pub body: InterfaceBody,
    pub attachments: Vec<InterfaceUrl>, // URLs or paths to attachments
    pub review_comments: Vec<AutherReviewCommentResponse>,
    pub rejection_reason: Option<InterfaceComment>,
}

pub struct AutherReviewCommentResponse {
    pub reviewer_id: InterfaceUserId,
    pub comment: InterfaceComment,
    pub created_at: InterfaceDateTime,
}

/// Mappers from Domain

impl TryFrom<AutherReportOutput> for AutherReportResponse {
    type Error = InterfaceError;

    fn try_from(value: AutherReportOutput) -> InterfaceResult<Self> {
        Ok(Self {
            title: value.title.try_into()?,
            content: value.content.try_into()?,
            report_type: value.report_type.into(),
            permissions: value
                .permissions
                .into_iter()
                .map(|permission| permission.into())
                .collect(),
            status: value.status.into(),
            author_id: value.author_id.into(),
            assigned_reviewer_id: value
                .assigned_reviewer_id
                .into_iter()
                .map(|reviwer| reviwer.into())
                .collect(),
            due_date: value.due_date.map(|a| a.try_into()).transpose()?,
            version: value.version,
            id: value.id.into(),
            created_at: value.created_at.try_into()?,
            updated_at: value.updated_at.try_into()?,
        })
    }
}

impl TryFrom<AutherReportContentOutput> for AutherReportContentResponse {
    type Error = InterfaceError;

    fn try_from(value: AutherReportContentOutput) -> InterfaceResult<Self> {
        Ok(Self {
            body: value.body.try_into()?,
            attachments: {
                let mut attachments = Vec::new();
                for url in value.attachments.into_iter() {
                    attachments.push(url.try_into()?);
                }
                attachments
            },
            review_comments: {
                let mut review_comments = Vec::new();
                for comment in value.review_comments.into_iter() {
                    review_comments.push(comment.try_into()?);
                }
                review_comments
            },
            rejection_reason: value.rejection_reason.map(|a| a.try_into()).transpose()?,
        })
    }
}

impl TryFrom<AutherReviewCommentOutput> for AutherReviewCommentResponse {
    type Error = InterfaceError;

    fn try_from(value: AutherReviewCommentOutput) -> InterfaceResult<Self> {
        Ok(Self {
            reviewer_id: value.reviewer_id.into(),
            comment: value.comment.try_into()?,
            created_at: value.created_at.try_into()?,
        })
    }
}
