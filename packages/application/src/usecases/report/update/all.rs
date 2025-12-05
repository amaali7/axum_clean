use domain::Report;

use crate::{ dto::report_dto::{input::UpdateReportInput, output::PreivilegeReportOutput}, error::AppResult, ports::ReportRepository};

pub struct UpdateReportUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> UpdateReportUseCase<R> {
    pub async fn execute(&self, input: UpdateReportInput) -> AppResult<PreivilegeReportOutput> {
        let report = Report::try_from(input)?; 
        self.repo.update(&report).await?;
        Ok(PreivilegeReportOutput::from(report))
    }
}
