use domain::{events::UserEvent, Email, Permission, RoleId, User, UserId, Username};

use crate::error::AppResult;


#[async_trait::async_trait]
pub trait UserRepository {
    async fn save(&self, user: User) -> AppResult<()>;
    async fn update(&self, user: User) -> AppResult<()>;
    async fn get_by_id(&self, id: UserId) -> AppResult<Option<User>>;
    async fn delete(&self, id: UserId) -> AppResult<()>;
    async fn get_by_email(&self, id: Email) -> AppResult<Option<User>>;
    async fn get_by_username(&self, id: Username) -> AppResult<Option<User>>;
    async fn get_events(&self, user_id: UserId) -> AppResult<Option<Vec<UserEvent>>>;
    async fn list(&self) -> AppResult<Vec<User>>;
    async fn assign_permission(&self, user_id: UserId, permission: Permission) -> AppResult<()>;
    async fn remove_permission(&self, user_id: UserId, permission: Permission) -> AppResult<()>;
    async fn assign_role(&self, user_id: UserId, role_id: RoleId) -> AppResult<()>;
    async fn remove_role(&self, user_id: UserId, role_id: RoleId) -> AppResult<()>;
}
