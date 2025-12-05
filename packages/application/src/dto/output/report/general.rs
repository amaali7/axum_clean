use domain::{
    value_objects::{Body, Title, Url},
    DateTime, Report, ReportContent, ReportId, ReportType, UserId,
};

use crate::error::ApplicationError;
/// General Report Output
pub struct GeneralReportOutput {
    pub id: ReportId,
    pub title: Title,
    pub content: GeneralReportContentOutput,
    pub report_type: ReportType,
    pub author_id: UserId,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub due_date: Option<DateTime>,
    pub version: u64,
}
pub struct GeneralReportContentOutput {
    pub body: Body,
    pub attachments: Vec<Url>, // URLs or paths to attachments
}

/// Mappers from Domain

impl From<Report> for GeneralReportOutput {
    fn from(value: Report) -> Self {
        Self {
            id: value.id(),
            title: value.title(),
            content: GeneralReportContentOutput::from(value.content()),
            report_type: value.report_type(),
            author_id: value.author_id(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
            due_date: value.due_date(),
            version: value.version(),
        }
    }
}

impl From<ReportContent> for GeneralReportContentOutput {
    fn from(value: ReportContent) -> Self {
        Self {
            body: value.body(),
            attachments: value.attachments(),
        }
    }
}

/// Mapper to Domain
impl TryFrom<GeneralReportOutput> for Report {
    type Error = ApplicationError;

    fn try_from(value: GeneralReportOutput) -> Result<Self, Self::Error> {
        let mut builder = Self::new(value.id, value.author_id);
        builder
            .set_report_type(value.report_type)
            .set_due(value.due_date.unwrap_or_default())
            .set_content(ReportContent::try_from(value.content)?);
        let report = builder.build(&value.title, value.created_at)?;
        Ok(report)
    }
}

impl TryFrom<GeneralReportContentOutput> for ReportContent {
    type Error = ApplicationError;

    fn try_from(value: GeneralReportContentOutput) -> Result<Self, Self::Error> {
        let mut builder = Self::new();
        builder.set_body(value.body);
        for attachment in value.attachments.into_iter() {
            builder.add_attachment(attachment);
        }
        let report_content = builder.build()?;
        Ok(report_content)
    }
}
