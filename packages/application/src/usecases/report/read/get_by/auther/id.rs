
use domain::ReportId;

use crate::{ SubjectContex, dto::report_dto::output::AutherReportOutput, error::AppResult, ports::ReportRepository};


pub struct GetReportByIdAutherUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByIdAutherUseCase<R> {
    pub async fn execute(&self, ctx: SubjectContex, report_id: ReportId) -> AppResult<AutherReportOutput> {
        Ok(self.repo.get_by_id( ctx,report_id.clone()).await?.into())    }
}
