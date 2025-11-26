use domain::{events::ReportEvent, value_objects::Title, Name, Permission, Report, ReportId, Role, RoleId, UserId};

use crate::error::ApplicationError;

#[async_trait::async_trait]
pub trait ReportRepository {
    async fn save(&self, report: &Report) -> Result<(), ApplicationError>;
    async fn update(&self,report_id: &ReportId, report: &Report) -> Result<(), ApplicationError>;
    async fn get_by_id(&self, id: &ReportId) -> Result<Option<Report>, ApplicationError>;
    async fn get_by_author_id(&self, id: &UserId) -> Result<Option<Report>, ApplicationError>;
    async fn get_by_title(&self, title: &Title) -> Result<Option<Report>, ApplicationError>;
    async fn get_events(&self, report_id: &ReportId) -> Result<Option<Vec<ReportEvent>>, ApplicationError>;
    async fn list(&self) -> Result<Vec<Report>, ApplicationError>;
    async fn assign_permission(&self, role_id: &RoleId, permission: &Permission) -> Result<(), ApplicationError>;
    async fn remove_permission(&self, role_id: &RoleId, permission: &Permission) -> Result<(), ApplicationError>;
    async fn assign_reviewer(&self, report_id: &ReportId, reviewer_id: &UserId) -> Result<(), ApplicationError>;
    async fn remove_reviewer(&self, report_id: &ReportId, reviewer_id: &UserId) -> Result<(), ApplicationError>;
}
