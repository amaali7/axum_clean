use domain::{Name, Permission, Role, RoleId};

use crate::error::AppResult;

#[async_trait::async_trait]
pub trait RoleRepository {
    async fn save(&self, role: Role) -> AppResult<()>;
    async fn update(&self, role: Role) -> AppResult<()>;
    async fn get_by_id(&self, id: RoleId) -> AppResult<Option<Role>>;
    async fn get_by_name(&self, id: Name) -> AppResult<Option<Role>>;
    async fn list(&self) -> AppResult<Vec<Role>>;
    async fn delete(&self, id: RoleId) -> AppResult<()>;
    async fn assign_permission(&self, role_id: RoleId, permission: Permission) -> AppResult<()>;
    async fn remove_permission(&self, role_id: RoleId, permission: Permission) -> AppResult<()>;
}
