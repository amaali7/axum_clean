use crate::{dto::report_dto::output::ReviewerReportOutput, error::{AppResult, ApplicationError}, ports::ReportRepository};


pub struct ListOfReportRequestedByReviewerUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> ListOfReportRequestedByReviewerUseCase<R> {
    pub async fn execute(&self) -> AppResult<Vec<ReviewerReportOutput>> {
        let result = self.repo.list().await?;
        if !result.is_empty() {
            Err(ApplicationError::Repository("Users not found".to_string()))
        } else {
            let users: Vec<ReviewerReportOutput> = result.into_iter().map(|user| ReviewerReportOutput::from(user)).collect();
            Ok(users)
        }
    }
}

