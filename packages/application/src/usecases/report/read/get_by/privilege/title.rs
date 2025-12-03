use domain::Title;

use crate::{dto::report_dto::output::PreivilegeReportOutput, error::{AppResult, ApplicationError}, ports::ReportRepository};



pub struct GetReportByTitlePrivilegeUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByTitlePrivilegeUseCase<R> {
    pub async fn execute(&self, title: Title) -> AppResult<PreivilegeReportOutput> {
        let result = self.repo.get_by_title(&title).await?;
        match result {
            Some(report) => Ok(PreivilegeReportOutput::from(report)),
            None => Err(ApplicationError::Repository(format!("Report : title {} not found", title))),
        }
    }
}
