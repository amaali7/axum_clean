
use async_trait::async_trait;
use surrealdb::RecordId;

use application::{error::{AppResult, ApplicationError}, ports::{OrderBy, UserRepository}};
use domain::{Email, Username, events::DomainEvent, user::{User, UserId}};

use crate::{error::InfrastructureError, serialization::{events::SerializedEvent, user::user::SerializedUser}, surreal::client::SurrealDBClient};

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
    async fn save(&self, user: User) -> AppResult<Option<User>>{
        let record: SerializedUser = user.try_into().map_err(|_| ApplicationError::Repository("User Error ".to_string()))?;
        let user: Option<SerializedUser> =  self
            .client
            .db
            .create("user")
            .content(record)
            .await
            .map_err(|err| {
                let error: InfrastructureError = err.into();
                error
            })?;
        match user {
            Some(output) => Ok(Some(User::try_from(output)?)),
            None => Ok(None),
        } 
        
    }
    async fn update(&self, user: User) -> AppResult<Option<User>>{
        let record: SerializedUser = user.try_into().map_err(|_| ApplicationError::Repository("User Error ".to_string()))?;
        let mut user: Vec<SerializedUser> =  self
            .client
            .db
            
            .update("user")
            .content(record)
            .await
            .map_err(InfrastructureError::from)?;
            
        if let Some(user) =  user.pop(){
            let output: User = user.try_into()?;
            return Ok(Some(output));
        }else {
            return Ok(None);
        }
    }
    async fn get_by_id(&self, id: UserId) -> AppResult<Option<User>>{
        let user_id = RecordId::from(("user", id.as_str()));
         let maybe_user: Option<SerializedUser> = self
            .client
            .db
            .select(user_id)
            .await
            .map_err(|e| InfrastructureError::Surreal(e))?; // Convert SurrealDB error
        let user = match maybe_user {
            Some(ser_user) => Some( User::try_from(ser_user).map_err(InfrastructureError::from)?),
            None => None,
        };
        Ok(user)
    }
    async fn delete(&self, id: UserId) -> AppResult<bool>{
        let user: Option<SerializedUser> =  self
            .client
            .db
            .delete(("user", id.to_string()))
            .await
            .map_err(InfrastructureError::from)?;
        Ok(user.is_some())
    }
     async fn get_by_email(&self, id: Email) -> AppResult<Option<User>>{
         let maybe_user: Option<SerializedUser> = self
            .client
            .db
            .query("SELECT * FROM user WHERE email = $email")
            .bind(("email", id.to_string()))
            .await
            .map_err(|e| InfrastructureError::Surreal(e))?
             .take(0)
             .map_err(|e| InfrastructureError::Surreal(e))?; // Convert SurrealDB error
        let user = match maybe_user {
            Some(ser_user) => Some( User::try_from(ser_user).map_err(InfrastructureError::from)?),
            None => None,
        };
        Ok(user)
    }
    async fn get_by_username(&self, id: Username) -> AppResult<Option<User>>{
         let maybe_user: Option<SerializedUser> = self
            .client
            .db
            .query("SELECT * FROM user WHERE email = $email")
            .bind(("email", id.to_string()))
            .await
            .map_err(|e| InfrastructureError::Surreal(e))?
             .take(0)
             .map_err(|e| InfrastructureError::Surreal(e))?; // Convert SurrealDB error
        let user = match maybe_user {
            Some(ser_user) => Some( User::try_from(ser_user).map_err(InfrastructureError::from)?),
            None => None,
        };
        Ok(user)
    }
    async fn get_events(&self, order_by: Option<OrderBy>, user_id: UserId) -> AppResult<Option<Vec<DomainEvent>>>{
        let query = match order_by {
            Some(ordered) =>{
                let (order_by, sort) = ordered.result();
                format!("SELECT * FROM events ORDER BY {} {} WHERE occurred_by = $occurred_by", order_by, sort).as_str()
            },
            None => "SELECT * FROM events WHERE occurred_by = $occurred_by"
        };
         let maybe_events: Vec<SerializedEvent> = self
            .client
            .db
            .query(query)
            .bind(("occurred_by", user_id.to_string()))
            .await
            .map_err(|e| InfrastructureError::Surreal(e))?
             .take(0)
             .map_err(|e| InfrastructureError::Surreal(e))?; // Convert SurrealDB error
        if maybe_events.is_empty() {
            return Ok(None);
        }else {
            let mut events: Vec<DomainEvent> = Vec::new();
            for event in maybe_events.into_iter() {
                events.push(event.try_into()?);
            }
            return Ok(Some(events));
        }
    }
    async fn get_users_paginated(&self, page: u32, page_size: u32) -> AppResult<Vec<User>>{
        let offset = (page - 1) * page_size;
        
        let query = match order_by {
            Some(ordered) =>{
                let (order_by, sort) = ordered.result();
                format!("SELECT * FROM events ORDER BY {} {} WHERE occurred_by = $occurred_by", order_by, sort).as_str()
            },
            None => "SELECT * FROM events WHERE occurred_by = $occurred_by"
        };
        let mut response = self
            .client
            .db
            .query("SELECT * FROM user ORDER BY created_at DESC LIMIT $limit START $start")
            .bind(("limit", page_size))
            .bind(("start", offset))
            .await
            .map_err(InfrastructureError::from)?;

        let users: Vec<SerializedUser> = response
            .take(0)
            .map_err(|e| InfrastructureError::Repository(format!("Failed to parse users: {}", e)))?;

        Ok(users)
    }
    async fn assign_permission(&self, user_id: UserId, permission: Permission) -> AppResult<bool>;
    async fn remove_permission(&self, user_id: UserId, permission: Permission) -> AppResult<bool>;
    async fn assign_role(&self, user_id: UserId, role_id: RoleId) -> AppResult<bool>;
    async fn remove_role(&self, user_id: UserId, role_id: RoleId) -> AppResult<bool>;
    }


