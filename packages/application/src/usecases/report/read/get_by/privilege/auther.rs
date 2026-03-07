use domain::UserId;

use crate::{ SubjectContex, dto::report_dto::output::PreivilegeReportOutput, error::{AppResult, AppError}, ports::{ReportRepository, SortBy}};



pub struct GetReportByAutherPrivilegeUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByAutherPrivilegeUseCase<R> {
    pub async fn execute(&self,ctx: SubjectContex,sort_by: &[SortBy], page: u32, page_size: u32, auther_id: UserId ) -> AppResult<Vec<PreivilegeReportOutput>> {
        let result = self.repo.get_by_author_id(ctx, sort_by,page, page_size, auther_id.clone()).await?;
           if !result.is_empty() {
            Err(AppError::Repository("Reports not found".to_string()))
        } else {
            let reports: Vec<PreivilegeReportOutput> = result.into_iter().map(|report| PreivilegeReportOutput::from(report)).collect();
            Ok(reports)
        }
    }
}
