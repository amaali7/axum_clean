use domain::Title;

use crate::{SubjectContex, dto::report_dto::output::PreivilegeReportOutput, error::AppResult, ports::ReportRepository};



pub struct GetReportByTitlePrivilegeUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByTitlePrivilegeUseCase<R> {
    pub async fn execute(&self, ctx: SubjectContex, title: Title) -> AppResult<PreivilegeReportOutput> {
        Ok(self.repo.get_by_title( ctx,title.clone()).await?.into())
    }
}
