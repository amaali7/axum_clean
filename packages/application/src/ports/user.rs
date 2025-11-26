use domain::{events::UserEvent, Email, Permission, RoleId, User, UserId, Username};

use crate::error::ApplicationError;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn save(&self, user: &User) -> Result<(), ApplicationError>;
    async fn update(&self,user_id: &UserId, user: &User) -> Result<(), ApplicationError>;
    async fn get_by_id(&self, id: &UserId) -> Result<Option<User>, ApplicationError>;
    async fn get_by_email(&self, id: &Email) -> Result<Option<User>, ApplicationError>;
    async fn get_by_username(&self, id: &Username) -> Result<Option<User>, ApplicationError>;
    async fn get_events(&self, user_id: &UserId) -> Result<Option<Vec<UserEvent>>, ApplicationError>;
    async fn list(&self) -> Result<Vec<User>, ApplicationError>;
    async fn assign_permission(&self, user_id: UserId, permission: Permission) -> Result<(), ApplicationError>;
    async fn remove_permission(&self, user_id: UserId, permission: Permission) -> Result<(), ApplicationError>;
    async fn assign_role(&self, user_id: UserId, role_id: RoleId) -> Result<(), ApplicationError>;
    async fn remove_role(&self, user_id: UserId, role_id: RoleId) -> Result<(), ApplicationError>;
}
