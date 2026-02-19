
use domain::ReportId;

use crate::{RequestContex, dto::report_dto::output::PreivilegeReportOutput, error::AppResult, ports::ReportRepository};



pub struct GetReportByIdPrivilegeUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByIdPrivilegeUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, report_id: ReportId) -> AppResult<PreivilegeReportOutput> {
        Ok(self.repo.get_by_id( ctx,report_id.clone()).await?.into())    }
}
