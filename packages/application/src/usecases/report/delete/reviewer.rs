use domain::{ReportId, UserId};

use crate::{error::AppResult, ports::ReportRepository};


pub struct RemoveReviewerFromReportUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> RemoveReviewerFromReportUseCase<R> {
    pub async fn execute(&self, report_id: ReportId , reviewer_id:UserId) -> AppResult<()> {
        self.repo.remove_reviewer(&report_id, &reviewer_id).await
    }
}
