use domain::{ReportId, UserId};

use crate::{error::AppResult, ports::ReportRepository};


pub struct AssignReviewerForReportUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> AssignReviewerForReportUseCase<R> {
    pub async fn execute(&self, report_id:ReportId, reviewer_id: UserId ) -> AppResult<()> {
        self.repo.assign_reviewer(report_id.clone(),reviewer_id.clone()).await?;
        Ok(())
    }
}
