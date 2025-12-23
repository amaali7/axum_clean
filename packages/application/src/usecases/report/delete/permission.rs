use domain::{Permission, ReportId};

use crate::{error::AppResult, ports::ReportRepository};


pub struct RemovePermissionFromReportUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> RemovePermissionFromReportUseCase<R> {
    pub async fn execute(&self, report_id: ReportId , permission:Permission) -> AppResult<()> {
        self.repo.remove_permission(report_id, permission).await
    }
}
