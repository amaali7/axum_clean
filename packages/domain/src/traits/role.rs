use async_trait::async_trait;

use crate::{DomainError, Role, RoleId};

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn find_by_id(&self, id: &RoleId) -> Result<Role, DomainError>;
    async fn find_by_name(&self, name: &str) -> Result<Role, DomainError>;
    async fn find_all(&self) -> Result<Vec<Role>, DomainError>;
    async fn save(&self, role: &Role) -> Result<(), DomainError>;
}
