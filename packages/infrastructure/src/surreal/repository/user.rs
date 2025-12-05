use application::{error::AppResult, ports::UserRepository};
use async_trait::async_trait;
use domain::{events::UserEvent, Email, Permission, RoleId, User, UserId, Username};
use surrealdb::sql;

use crate::surreal::client::SurrealDBClient;


pub struct SurrealUserRepository {
    client: SurrealDBClient,
}

impl SurrealUserRepository {
    pub fn new(client: SurrealDBClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl UserRepository for SurrealUserRepository {
    async fn save(&self, user: &User) -> AppResult<()>{
        Ok(())
    }
    async fn update(&self, user: &User) -> AppResult<()>{
        Ok(())
    }
    async fn get_by_id(&self, id: &UserId) -> AppResult<Option<User>>{
        Ok(None)
    }
    async fn delete(&self, id: &UserId) -> AppResult<()>{
        Ok(())
    }
    async fn get_by_email(&self, id: &Email) -> AppResult<Option<User>>{
        Ok(None)
    }
    async fn get_by_username(&self, id: &Username) -> AppResult<Option<User>>{
        Ok(None)
    }
    async fn get_events(&self, user_id: &UserId) -> AppResult<Option<Vec<UserEvent>>>{
        Ok(None)
    }
    async fn list(&self) -> AppResult<Vec<User>>{
        let vecto: Vec<User> = Vec::new();
        Ok(vecto)
    }
    async fn assign_permission(&self, user_id: UserId, permission: Permission) -> AppResult<()>{
        Ok(())
    }
    async fn remove_permission(&self, user_id: UserId, permission: Permission) -> AppResult<()>{
        Ok(())
    }
    async fn assign_role(&self, user_id: UserId, role_id: RoleId) -> AppResult<()>{
        Ok(())
    }
    async fn remove_role(&self, user_id: UserId, role_id: RoleId) -> AppResult<()>{
        Ok(())
    }
}
