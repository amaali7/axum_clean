use domain::{Name, RoleId};

use crate::{SubjectContex, dto::role::{command::RoleCommand, view::RoleView}, error::AppResult};

use super::SortBy;


#[derive(Debug, Clone)]
pub enum RoleQueryResult {
    Single(RoleView),
    Array(Vec<RoleView>),
    None,
}

impl RoleQueryResult {
    pub fn get_array(&self) -> Option<Vec<RoleView>> {
        match self {
            RoleQueryResult::Array(roles) => Some(roles.to_vec()),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<RoleView> {
        match self {
            RoleQueryResult::Single(role) => Some(role.clone()),
            _ => None,
        }
    }
}


#[async_trait::async_trait]
pub trait RoleRepository {
    async fn create(&self, ctx: SubjectContex, role: RoleCommand) -> AppResult<RoleView>;
    async fn update(&self, ctx: SubjectContex, role: RoleCommand) -> AppResult<RoleView>;
    async fn get_by_id(&self, ctx: SubjectContex, id: RoleId) -> AppResult<RoleView>;
    async fn get_by_name(&self, ctx: SubjectContex, id: Name) -> AppResult<RoleView>;
    async fn get_roles_paginated(&self,ctx: SubjectContex,sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<RoleView>>;
    async fn delete(&self, ctx: SubjectContex, id: RoleId) -> AppResult<bool>;
    async fn raw_query(&self,ctx: SubjectContex, query: String) -> AppResult<RoleQueryResult>;
}
