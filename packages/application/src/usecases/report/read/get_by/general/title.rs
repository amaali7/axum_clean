
use domain::Title;

use crate::{SubjectContex, dto::report_dto::output::GeneralReportOutput, error::AppResult, ports::ReportRepository};


pub struct GetReportByTitleGeneralUseCase<R: ReportRepository> {
    repo: R,
}

impl<R: ReportRepository> GetReportByTitleGeneralUseCase<R> {
    pub async fn execute(&self, ctx: SubjectContex, title: Title) -> AppResult<GeneralReportOutput> {
        Ok(self.repo.get_by_title( ctx,title.clone()).await?.into())
    }
}
