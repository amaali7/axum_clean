
use domain::ReportId;

use crate::{RequestContex, dto::report_dto::output::GeneralReportOutput, error::AppResult, ports::ReportRepository};

pub struct GetReportByIdGenaralUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByIdGenaralUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, report_id: ReportId) -> AppResult<GeneralReportOutput> {
        Ok(self.repo.get_by_id( ctx,report_id.clone()).await?.into())
    }
}
