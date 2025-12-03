use domain::UserId;

use crate::{ dto::report_dto::output::PreivilegeReportOutput, error::{AppResult, ApplicationError}, ports::ReportRepository};



pub struct GetReportByAutherPrivilegeUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByAutherPrivilegeUseCase<R> {
    pub async fn execute(&self, auther_id: UserId ) -> AppResult<PreivilegeReportOutput> {
        let result = self.repo.get_by_author_id(&auther_id).await?;
        match result {
            Some(report) => Ok(PreivilegeReportOutput::from(report)),
            None => Err(ApplicationError::Repository(format!("Report : by {} not found", auther_id))),
        }
    }
}


