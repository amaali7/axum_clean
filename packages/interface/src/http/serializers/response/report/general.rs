use application::dto::report_dto::output::{GeneralReportContentOutput, GeneralReportOutput};

use crate::{
    common_objects::{
        report::{report_type::InterfaceReportType, InterfaceReportId},
        user::InterfaceUserId,
    },
    error::{InterfaceError, InterfaceResult},
    value_objects::{InterfaceBody, InterfaceDateTime, InterfaceTitle, InterfaceUrl},
};
/// General Report Response
pub struct GeneralReportResponse {
    pub id: InterfaceReportId,
    pub title: InterfaceTitle,
    pub content: GeneralReportContentResponse,
    pub report_type: InterfaceReportType,
    pub author_id: InterfaceUserId,
    pub created_at: InterfaceDateTime,
    pub updated_at: InterfaceDateTime,
    pub due_date: Option<InterfaceDateTime>,
    pub version: u64,
}
pub struct GeneralReportContentResponse {
    pub body: InterfaceBody,
    pub attachments: Vec<InterfaceUrl>, // URLs or paths to attachments
}

/// Mappers from Domain

impl TryFrom<GeneralReportOutput> for GeneralReportResponse {
    type Error = InterfaceError;

    fn try_from(value: GeneralReportOutput) -> InterfaceResult<Self> {
        Ok(Self {
            id: value.id.into(),
            title: value.title.try_into()?,
            content: value.content.try_into()?,
            report_type: value.report_type.into(),
            author_id: value.author_id.into(),
            created_at: value.created_at.try_into()?,
            updated_at: value.updated_at.try_into()?,
            due_date: value.due_date.map(|x| x.try_into()).transpose()?,
            version: value.version,
        })
    }
}

impl TryFrom<GeneralReportContentOutput> for GeneralReportContentResponse {
    type Error = InterfaceError;

    fn try_from(value: GeneralReportContentOutput) -> Result<Self, Self::Error> {
        Ok(Self {
            body: value.body.try_into()?,
            attachments: {
                let mut attachments = Vec::new();
                for url in value.attachments.into_iter() {
                    attachments.push(url.try_into()?);
                }
                attachments
            },
        })
    }
}
