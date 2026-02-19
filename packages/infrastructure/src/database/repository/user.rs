use async_trait::async_trait;
use application::{RequestContex, error::{AppResult, ApplicationError}, ports::{ SortBy, UserRepository, user::UserQueryResult}};
use domain::{Email, Username, user::{User, UserId}};

use crate::{
    database::client::SurrealDBClient, error::InfrastructureError,
    serialization::{
        SerializedUserId,
        user::{
            SurrealUserResponseExt ,SerializedUserQueryResult,
            user::SerializedUser
        },
        value_objects::{
            SerializedEmail, SerializedUsername
        }
    }
};

pub struct SurrealUserRepository {
    client: SurrealDBClient,
}

impl SurrealUserRepository {
    pub fn new(client: SurrealDBClient) -> Self {
        Self { client }
    }
}

// TODO: Permission must have logic for ranking it

#[async_trait]
impl UserRepository for SurrealUserRepository {
    async fn create(&self,ctx: RequestContex, user: User) -> AppResult<User>{
        let record: SerializedUser = user.try_into().map_err(|_| ApplicationError::Repository("User Error ".to_string()))?;
        let result: Option<SerializedUser> =  self
            .client
            .db
            .query("LET $user_id = $uid;
                    INSERT INTO user CONTENT $user RETURN AFTER")
            .bind(("uid", ctx.user_id_as_str()))
            .bind(("user", record))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(user) => Ok(user.try_into()?),
            None => Err(ApplicationError::Repository("User not created!".to_string())),
        }
        
    }
    async fn update(&self,ctx: RequestContex, user: User) -> AppResult<User>{
        let record: SerializedUser = user.try_into().map_err(|err: InfrastructureError| ApplicationError::Domain(err.into()))?;
        let result: Option<SerializedUser> =  self
            .client
            .db
            .query("LET $user_id = $uid;
                    UPDATE user:$id MERGE $user RETURN AFTER")
            .bind(("user", record.clone()))
            .bind(("uid", ctx.user_id_as_str()))
            .bind(("id", record.id()))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(user) => Ok(user.try_into()?),
            None => Err(ApplicationError::Repository("User not Updated!".to_string())),
        }
    }
    async fn get_by_id(&self, _request_contex: RequestContex, id: UserId) -> AppResult<User>{
        let id: SerializedUserId = id.into();
        let result: Option<SerializedUser> =  self
            .client
            .db
            .query("SELECT * FROM ONLY user:$id")
            .bind(("id", id))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(user) => Ok(user.try_into()?),
            None => Err(ApplicationError::Repository("User not Updated!".to_string())),
        }
    }
    async fn delete(&self,ctx: RequestContex, id: UserId) -> AppResult<bool>{
        let result: Option<SerializedUser> =  self
            .client
            .db
            .query("LET $user_id = $uid;
                    DELETE person:$id RETURN BEFORE")
            .bind(("uid", ctx.user_id_as_str()))
            .bind(("id", id.id()))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(_) => Ok(true),
            None => Err(ApplicationError::Repository("User not deleted!".to_string())),
        }
    }
    async fn get_by_email(&self, _request_contex:RequestContex, email: Email) -> AppResult<User>{
        let email: SerializedEmail = email.try_into()?;
        let result: Option<SerializedUser> =  self
            .client
            .db
            .query("SELECT * FROM user WHERE email = $email LIMIT 1")
            .bind(("email", email))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(user) => Ok(user.try_into()?),
            None => Err(ApplicationError::Repository("User not Updated!".to_string())),
        }
    }
    async fn get_by_username(&self,_request_contex:RequestContex,  username: Username) -> AppResult<User>{
        let username: SerializedUsername = username.try_into()?;
        let result: Option<SerializedUser> =  self
            .client
            .db
            .query("SELECT * FROM user WHERE username = $username LIMIT 1")
            .bind(("username", username))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0)  // Get first query result
            .map_err(|err| ApplicationError::Repository(err.to_string()))?;
        match result {
            Some(user) => Ok(user.try_into()?),
            None => Err(ApplicationError::Repository("User not Updated!".to_string())),
        }
    }
    
    // TODO: Add permissions field into requert contex to be prossesed as required take into account murge all roles permessions with the user ones
    async fn get_users_paginated(&self,ctx: RequestContex ,sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<User>>{
        let mut order = String::new();
        
        for ord in sort_by{
            order.push_str(&format!("{},", ord.as_string()));
        }
        if !sort_by.is_empty(){
            order = format!("ORDER BY {}", order);
            order.truncate(order.len() -1);
        }
        
        let result: Vec<SerializedUser> =  self
            .client
            .db
            .query("SELECT * FROM user $order LIMIT $page_size START $start_at")
            .bind(("order", order))
            .bind(("page_size", page_size))
            .bind(("start_at", page*page_size))
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?.take(0).map_err(|err| ApplicationError::Repository(err.to_string()))?;
        let mut users: Vec<User> = Vec::new();
        for user in result{
            users.push(user.try_into().map_err(|err: InfrastructureError| ApplicationError::Repository(err.to_string()))?);
        }
        Ok(users)
    }

    async fn raw_query(&self,ctx: RequestContex, query: String) -> AppResult<UserQueryResult>{
        let response = self.client
            .db
            .query(query)
            .await.map_err(|err| ApplicationError::Repository(err.to_string()))?;
        // Using the extension trait
        Ok(response.into_user_result().await.map_err(|err| ApplicationError::Repository(err.to_string()))?.try_into()?)
    }
}


