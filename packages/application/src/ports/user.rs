use domain::{Email, Permission, RoleId, User, UserId, Username, events::DomainEvent};

use crate::error::AppResult;

use super::OrderBy;


#[async_trait::async_trait]
pub trait UserRepository {
    async fn save(&self, user: User) -> AppResult<Option<User>>;
    async fn update(&self, user: User) -> AppResult<Option<User>>;
    async fn get_by_id(&self, id: UserId) -> AppResult<Option<User>>;
    async fn delete(&self, id: UserId) -> AppResult<bool>;
    async fn get_by_email(&self, id: Email) -> AppResult<Option<User>>;
    async fn get_by_username(&self, id: Username) -> AppResult<Option<User>>;
    async fn get_events(&self,order_by: OrderBy, user_id: UserId) -> AppResult<Option<Vec<DomainEvent>>>;
    async fn get_users_paginated(&self,order_by: OrderBy, page: u32, page_size: u32) -> AppResult<Vec<User>>;
    async fn assign_permission(&self, user_id: UserId, permission: Permission) -> AppResult<bool>;
    async fn remove_permission(&self,user_id: UserId, permission: Permission) -> AppResult<bool>;
    async fn assign_role(&self, user_id: UserId, role_id: RoleId) -> AppResult<bool>;
    async fn remove_role(&self, user_id: UserId, role_id: RoleId) -> AppResult<bool>;
}
