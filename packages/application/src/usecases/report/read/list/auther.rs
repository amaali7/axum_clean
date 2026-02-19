use crate::{ RequestContex, dto::report_dto::output::AutherReportOutput, error::{AppResult, ApplicationError}, ports::{ReportRepository, SortBy}};



pub struct ListOfReportRequestedByAutherUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> ListOfReportRequestedByAutherUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, sort_by: &[SortBy], page: u32, page_size: u32) ->AppResult<Vec<AutherReportOutput>> {
        let result = self.repo.get_reports_paginated(ctx, sort_by,page, page_size).await?;
        if !result.is_empty() {
            Err(ApplicationError::Repository("Users not found".to_string()))
        } else {
            let users: Vec<AutherReportOutput> = result.into_iter().map(|user| AutherReportOutput::from(user)).collect();
            Ok(users)
        }
    }
}
