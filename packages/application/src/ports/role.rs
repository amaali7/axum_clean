use domain::{Name, Permission, Role, RoleId, UserId};

use crate::error::ApplicationError;

#[async_trait::async_trait]
pub trait RoleRepository {
    async fn save(&self, role: &Role) -> Result<(), ApplicationError>;
    async fn update(&self,role_id: &RoleId, role: &Role) -> Result<(), ApplicationError>;
    async fn get_by_id(&self, id: &RoleId) -> Result<Option<Role>, ApplicationError>;
    async fn get_by_name(&self, id: &Name) -> Result<Option<Role>, ApplicationError>;
    async fn list(&self) -> Result<Vec<Role>, ApplicationError>;
    async fn assign_permission(&self, role_id: RoleId, permission: Permission) -> Result<(), ApplicationError>;
    async fn remove_permission(&self, role_id: RoleId, permission: Permission) -> Result<(), ApplicationError>;
}
