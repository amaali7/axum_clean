use domain::ReportId;

use crate::{SubjectContex, error::AppResult, ports::ReportRepository};


pub struct DeleteReportUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> DeleteReportUseCase<R> {
    pub async fn execute(&self, ctx: SubjectContex, report_id: ReportId) -> AppResult<bool> {
        self.repo.delete(ctx, report_id.clone()).await
    }
}
