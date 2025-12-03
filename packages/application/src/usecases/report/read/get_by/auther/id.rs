
use domain::ReportId;

use crate::{ dto::report_dto::output::AutherReportOutput, error::{AppResult, ApplicationError}, ports::ReportRepository};


pub struct GetReportByIdAutherUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByIdAutherUseCase<R> {
    pub async fn execute(&self, report_id: ReportId) -> AppResult<AutherReportOutput> {
        let result = self.repo.get_by_id(&report_id).await?;
        match result {
            Some(report) => Ok(AutherReportOutput::from(report)),
            None => Err(ApplicationError::Repository(format!("Report : id {} not found", report_id))),
        }
    }
}
