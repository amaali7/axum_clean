use domain::{report::content::ReviewComment, value_objects::{Comment, DateTime}, Report, ReportContent};

use crate::{
    dto::report_dto::{
        input::CreateReportInput,
        output::PreivilegeReportOutput
    }
    , error::AppResult, ports::ReportRepository
};

pub struct CreateReportUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> CreateReportUseCase<R> {
    pub async fn execute(&self, input: CreateReportInput) -> AppResult<PreivilegeReportOutput> {
        let mut report_content_builder = ReportContent::new();
        report_content_builder
            .set_body(input.content.body)
            .set_rejection_reason(input.content.rejection_reason.unwrap_or(Comment::default()));

        for review_comment in input.content.review_comments.into_iter() {
            report_content_builder.add_review_comment(
                ReviewComment::new(
                    review_comment.reviewer_id
                        , review_comment.comment
                        , review_comment.created_at
                )
            );
        }
        for attachment in input.content.attachments.into_iter() {
            report_content_builder.add_attachment(attachment);
        }
        let report_content = report_content_builder.build()?;

        let mut report_builder = Report::new(input.id, input.author_id);
        report_builder
            .set_content(report_content)
            .set_due(input.due_date.unwrap_or(DateTime::default()))
            .set_report_type(input.report_type);

        for permission in input.permissions.into_iter() {
             report_builder.add_permission(permission);
        }
        for event in input.events.into_iter() {
             report_builder.add_event(event);
        }
        for reviewer in input.assigned_reviewer_id.into_iter() {
             report_builder.add_reviewer(reviewer);
        }
        let report = report_builder
            .build(&input.title, input.created_at)?;

        self.repo.save(&report).await?;
        Ok(PreivilegeReportOutput::from(report))
    }
}
