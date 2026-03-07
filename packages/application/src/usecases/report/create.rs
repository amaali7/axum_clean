use domain::Report;

use crate::{
    SubjectContex, dto::report_dto::{
        input::CreateReportInput,
        output::PreivilegeReportOutput
    }, error::AppResult, ports::ReportRepository
};

pub struct CreateReportUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> CreateReportUseCase<R> {
    pub async fn execute(&self, ctx: SubjectContex, input: CreateReportInput) -> AppResult<PreivilegeReportOutput> {
        let report = Report::try_from(input)?;
        self.repo.create(ctx, report.clone()).await?;
        Ok(PreivilegeReportOutput::from(report))
    }
}
