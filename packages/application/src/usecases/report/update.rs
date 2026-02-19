use domain::Report;

use crate::{ RequestContex, dto::report_dto::{input::UpdateReportInput, output::PreivilegeReportOutput}, error::AppResult, ports::ReportRepository};

pub struct UpdateReportUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> UpdateReportUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, input: UpdateReportInput) -> AppResult<PreivilegeReportOutput> {
        let report = Report::try_from(input)?; 
        self.repo.update( ctx, report.clone()).await?;
        Ok(PreivilegeReportOutput::from(report))
    }
}
