
use domain::Title;

use crate::{dto::report_dto::output::GeneralReportOutput, error::{AppResult, ApplicationError}, ports::ReportRepository};


pub struct GetReportByTitleGeneralUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByTitleGeneralUseCase<R> {
    pub async fn execute(&self, title: Title) -> AppResult<GeneralReportOutput> {
        let result = self.repo.get_by_title(&title).await?;
        match result {
            Some(report) => Ok(GeneralReportOutput::from(report)),
            None => Err(ApplicationError::Repository(format!("Report : title {} not found", title))),
        }
    }
}
