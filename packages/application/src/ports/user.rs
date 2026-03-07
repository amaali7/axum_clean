use domain::{Email, User, UserId, Username};

use crate::{SubjectContex, error::AppResult};

use super::{ SortBy};

#[derive(Debug, Clone)]
pub enum UserQueryResult {
    Single(User),
    Array(Vec<User>),
    None,
}

impl UserQueryResult {
    pub fn get_array(&self) -> Option<Vec<User>> {
        match self {
            UserQueryResult::Array(users) => Some(users.to_vec()),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<User> {
        match self {
            UserQueryResult::Single(user) => Some(user.clone()),
            _ => None,
        }
    }
}

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create(&self,ctx: SubjectContex, user: User) -> AppResult<User>;
    async fn update(&self,ctx: SubjectContex, user: User) -> AppResult<User>;
    async fn get_by_id(&self,ctx: SubjectContex, id: UserId) -> AppResult<User>;
    async fn delete(&self,ctx: SubjectContex, id: UserId) -> AppResult<bool>;
    async fn get_by_email(&self,ctx: SubjectContex, email: Email) -> AppResult<User>;
    async fn get_by_username(&self,ctx: SubjectContex, username: Username) -> AppResult<User>;
    async fn get_users_paginated(&self,ctx: SubjectContex,sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<User>>;
    async fn raw_query(&self,ctx: SubjectContex, query: String) -> AppResult<UserQueryResult>;
}
