use domain::UserId;

use crate::{ RequestContex, dto::report_dto::output::PreivilegeReportOutput, error::{AppResult, ApplicationError}, ports::{ReportRepository, SortBy}};



pub struct GetReportByAutherPrivilegeUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByAutherPrivilegeUseCase<R> {
    pub async fn execute(&self,ctx: RequestContex,sort_by: &[SortBy], page: u32, page_size: u32, auther_id: UserId ) -> AppResult<Vec<PreivilegeReportOutput>> {
        let result = self.repo.get_by_author_id(ctx, sort_by,page, page_size, auther_id.clone()).await?;
           if !result.is_empty() {
            Err(ApplicationError::Repository("Reports not found".to_string()))
        } else {
            let reports: Vec<PreivilegeReportOutput> = result.into_iter().map(|report| PreivilegeReportOutput::from(report)).collect();
            Ok(reports)
        }
    }
}
