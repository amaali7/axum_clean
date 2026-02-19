
use domain::Title;

use crate::{ RequestContex, dto::report_dto::output::AutherReportOutput, error::AppResult, ports::ReportRepository};


pub struct GetReportByTitleAutherUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByTitleAutherUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, title: Title) -> AppResult<AutherReportOutput> {
        Ok(self.repo.get_by_title( ctx,title.clone()).await?.into())
    }
}
