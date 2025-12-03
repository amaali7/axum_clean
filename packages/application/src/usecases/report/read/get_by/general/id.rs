
use domain::ReportId;

use crate::{dto::report_dto::output::GeneralReportOutput, error::{AppResult, ApplicationError}, ports::ReportRepository};

pub struct GetReportByIdGenaralUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByIdGenaralUseCase<R> {
    pub async fn execute(&self, report_id: ReportId) -> AppResult<GeneralReportOutput> {
        let result = self.repo.get_by_id(&report_id).await?;
        match result {
            Some(report) => Ok(GeneralReportOutput::from(report)),
            None => Err(ApplicationError::Repository(format!("Report : id {} not found", report_id))),
        }
    }
}
