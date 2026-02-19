
use domain::UserId;

use crate::{ RequestContex, dto::report_dto::output::AutherReportOutput, error::{AppResult, ApplicationError}, ports::{ReportRepository, SortBy}};


pub struct GetReportByAutherIdAutherUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByAutherIdAutherUseCase<R> {
    pub async fn execute(&self,ctx: RequestContex,sort_by: &[SortBy], page: u32, page_size: u32, auther_id: UserId ) -> AppResult<Vec<AutherReportOutput>> {
        let result = self.repo.get_by_author_id(ctx, sort_by,page, page_size, auther_id.clone()).await?;
           if !result.is_empty() {
            Err(ApplicationError::Repository("Reports not found".to_string()))
        } else {
            let reports: Vec<AutherReportOutput> = result.into_iter().map(|report| AutherReportOutput::from(report)).collect();
            Ok(reports)
        }
            }
}

