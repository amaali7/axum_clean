use crate::{RequestContex, dto::report_dto::output::ReviewerReportOutput, error::{AppResult, ApplicationError}, ports::{ReportRepository, SortBy}};


pub struct ListOfReportRequestedByReviewerUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> ListOfReportRequestedByReviewerUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<ReviewerReportOutput>> {
        let result = self.repo.get_reports_paginated(ctx, sort_by,page, page_size).await?;
        if !result.is_empty() {
            Err(ApplicationError::Repository("Users not found".to_string()))
        } else {
            let users: Vec<ReviewerReportOutput> = result.into_iter().map(|user| ReviewerReportOutput::from(user)).collect();
            Ok(users)
        }
    }
}

