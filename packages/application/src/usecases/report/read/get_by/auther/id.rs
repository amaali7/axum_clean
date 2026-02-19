
use domain::ReportId;

use crate::{ RequestContex, dto::report_dto::output::AutherReportOutput, error::AppResult, ports::ReportRepository};


pub struct GetReportByIdAutherUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByIdAutherUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, report_id: ReportId) -> AppResult<AutherReportOutput> {
        Ok(self.repo.get_by_id( ctx,report_id.clone()).await?.into())    }
}
