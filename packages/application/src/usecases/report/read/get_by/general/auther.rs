
use domain::UserId;

use crate::{RequestContex, dto::report_dto::output::GeneralReportOutput, error::{AppResult, ApplicationError}, ports::{ReportRepository, SortBy}};


pub struct GetRebortByAutherGeneralCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetRebortByAutherGeneralCase<R> {
    pub async fn execute(&self,ctx: RequestContex,sort_by: &[SortBy], page: u32, page_size: u32, auther_id: UserId ) -> AppResult<Vec<GeneralReportOutput>> {
        let result = self.repo.get_by_author_id(ctx, sort_by,page, page_size, auther_id.clone()).await?;
           if !result.is_empty() {
            Err(ApplicationError::Repository("Reports not found".to_string()))
        } else {
            let reports: Vec<GeneralReportOutput> = result.into_iter().map(|report| GeneralReportOutput::from(report)).collect();
            Ok(reports)
        }
    }

}

