
use domain::UserId;

use crate::{ dto::report_dto::output::AutherReportOutput, error::{AppResult, ApplicationError}, ports::ReportRepository};


pub struct GetReportByAutherIdAutherUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByAutherIdAutherUseCase<R> {
    pub async fn execute(&self, auther_id: UserId ) -> AppResult<AutherReportOutput> {
        let result = self.repo.get_by_author_id(&auther_id).await?;
        match result {
            Some(user) => Ok(AutherReportOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("Report : not found"))),
        }
    }
}

