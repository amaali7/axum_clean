use domain::ReportId;

use crate::{error::AppResult, ports::ReportRepository};


pub struct DeleteReportUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> DeleteReportUseCase<R> {
    pub async fn execute(&self, report_id: ReportId) -> AppResult<()> {
        self.repo.delete(&report_id).await
    }
}
