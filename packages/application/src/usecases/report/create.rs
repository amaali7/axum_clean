use domain::Report;

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
        let report = Report::try_from(input)?;
        self.repo.save(&report).await?;
        Ok(PreivilegeReportOutput::from(report))
    }
}
