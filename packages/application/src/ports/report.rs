use domain::{events::ReportEvent, value_objects::Title, Permission, Report, ReportId, UserId};

use crate::error::AppResult;


#[async_trait::async_trait]
pub trait ReportRepository {
    async fn save(&self, report: &Report) -> AppResult<()>;
    async fn delete(&self, report_id: &ReportId) -> AppResult<()>;
    async fn update(&self, report: &Report) -> AppResult<Report>;
    async fn get_by_id(&self, id: &ReportId) -> AppResult<Option<Report>>;
    async fn get_by_author_id(&self, id: &UserId) -> AppResult<Option<Report>>;
    async fn get_by_title(&self, title: &Title) -> AppResult<Option<Report>>;
    async fn get_events(&self, report_id: &ReportId) -> AppResult<Option<Vec<ReportEvent>>>;
    async fn list(&self) -> AppResult<Vec<Report>>;
    async fn assign_permission(&self, report_id: &ReportId, permission: &Permission) -> AppResult<()>;
    async fn remove_permission(&self, report_id: &ReportId, permission: &Permission) -> AppResult<()>;
    async fn assign_reviewer(&self, report_id: &ReportId, reviewer_id: &UserId) -> AppResult<()>;
    async fn remove_reviewer(&self, report_id: &ReportId, reviewer_id: &UserId) -> AppResult<()>;
}
