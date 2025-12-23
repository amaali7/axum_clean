
use domain::Title;

use crate::{ dto::report_dto::output::AutherReportOutput, error::{AppResult, ApplicationError}, ports::ReportRepository};


pub struct GetReportByTitleAutherUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByTitleAutherUseCase<R> {
    pub async fn execute(&self, title: Title) -> AppResult<AutherReportOutput> {
        let result = self.repo.get_by_title(title.clone()).await?;
        match result {
            Some(report) => Ok(AutherReportOutput::from(report)),
            None => Err(ApplicationError::Repository(format!("Report : {} not found", title))),
        }
    }
}
