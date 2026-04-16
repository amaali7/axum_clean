use domain::{Email, UserId, Username};

use crate::{SubjectContex, dto::user::{command::UserCommand, view::UserView}, error::AppResult};

use super::{ SortBy};

#[derive(Debug, Clone)]
pub enum UserQueryResult {
    Single(UserView),
    Array(Vec<UserView>),
    None,
}

impl UserQueryResult {
    pub fn get_array(&self) -> Option<Vec<UserView>> {
        match self {
            UserQueryResult::Array(users) => Some(users.to_vec()),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<UserView> {
        match self {
            UserQueryResult::Single(user) => Some(user.clone()),
            _ => None,
        }
    }
}

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create(&self,ctx: SubjectContex, user: UserCommand) -> AppResult<UserView>;
    async fn update(&self,ctx: SubjectContex, user: UserCommand) -> AppResult<UserView>;
    async fn get_by_id(&self,ctx: SubjectContex, id: UserId) -> AppResult<UserView>;
    async fn delete(&self,ctx: SubjectContex, id: UserId) -> AppResult<bool>;
    async fn get_by_email(&self,ctx: SubjectContex, email: Email) -> AppResult<UserView>;
    async fn get_by_username(&self,ctx: SubjectContex, username: Username) -> AppResult<UserView>;
    async fn get_users_paginated(&self,ctx: SubjectContex,sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<UserView>>;
    async fn raw_query(&self,ctx: SubjectContex, query: String) -> AppResult<UserQueryResult>;
}
