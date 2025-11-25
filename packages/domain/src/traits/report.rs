use async_trait::async_trait;

use crate::{DomainError, Report, ReportId, ReportStatus, UserId};

#[async_trait]
pub trait ReportRepository: Send + Sync {
    async fn find_by_id(&self, id: &ReportId) -> Result<Report, DomainError>;
    async fn find_by_author(&self, author_id: &UserId) -> Result<Vec<Report>, DomainError>;
    async fn find_by_status(&self, status: ReportStatus) -> Result<Vec<Report>, DomainError>;
    async fn find_by_reviewer(&self, reviewer_id: &UserId) -> Result<Vec<Report>, DomainError>;
    async fn save(&self, report: &Report) -> Result<(), DomainError>;
    async fn delete(&self, id: &ReportId) -> Result<(), DomainError>;
}
