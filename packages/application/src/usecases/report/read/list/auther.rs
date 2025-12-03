use crate::{ dto::report_dto::output::AutherReportOutput, error::{AppResult, ApplicationError}, ports::ReportRepository};



pub struct ListOfReportRequestedByAutherUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> ListOfReportRequestedByAutherUseCase<R> {
    pub async fn execute(&self) ->AppResult<Vec<AutherReportOutput>> {
        let result = self.repo.list().await?;
        if !result.is_empty() {
            Err(ApplicationError::Repository("Users not found".to_string()))
        } else {
            let users: Vec<AutherReportOutput> = result.into_iter().map(|user| AutherReportOutput::from(user)).collect();
            Ok(users)
        }
    }
}
