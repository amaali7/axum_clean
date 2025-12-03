
use domain::UserId;

use crate::{dto::report_dto::output::GeneralReportOutput, error::{AppResult, ApplicationError}, ports::ReportRepository};


pub struct GetRebortByAutherGeneralCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetRebortByAutherGeneralCase<R> {
    pub async fn execute(&self, auther_id: UserId ) -> AppResult<GeneralReportOutput> {
        let result = self.repo.get_by_author_id(&auther_id).await?;
        match result {
            Some(report) => Ok(GeneralReportOutput::from(report)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", auther_id))),
        }
    }
}

