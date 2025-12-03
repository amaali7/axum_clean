
use domain::ReportId;

use crate::{dto::report_dto::output::PreivilegeReportOutput, error::{AppResult, ApplicationError}, ports::ReportRepository};



pub struct GetReportByIdPrivilegeUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByIdPrivilegeUseCase<R> {
    pub async fn execute(&self, report_id: ReportId) -> AppResult<PreivilegeReportOutput> {
        let result = self.repo.get_by_id(&report_id).await?;
        match result {
            Some(report) => Ok(PreivilegeReportOutput::from(report)),
            None => Err(ApplicationError::Repository(format!("Report : id {} not found", report_id))),
        }
    }
}
