use domain::{Name, Role, RoleId};

use crate::{RequestContex, error::AppResult};

use super::SortBy;


#[derive(Debug, Clone)]
pub enum RoleQueryResult {
    Single(Role),
    Array(Vec<Role>),
    None,
}

impl RoleQueryResult {
    pub fn get_array(&self) -> Option<Vec<Role>> {
        match self {
            RoleQueryResult::Array(roles) => Some(roles.to_vec()),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<Role> {
        match self {
            RoleQueryResult::Single(role) => Some(role.clone()),
            _ => None,
        }
    }
}


#[async_trait::async_trait]
pub trait RoleRepository {
    async fn create(&self, ctx: RequestContex, role: Role) -> AppResult<Role>;
    async fn update(&self, ctx: RequestContex, role: Role) -> AppResult<Role>;
    async fn get_by_id(&self, ctx: RequestContex, id: RoleId) -> AppResult<Role>;
    async fn get_by_name(&self, ctx: RequestContex, id: Name) -> AppResult<Role>;
    async fn get_roles_paginated(&self,ctx: RequestContex,sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<Role>>;
    async fn delete(&self, ctx: RequestContex, id: RoleId) -> AppResult<bool>;
    async fn raw_query(&self,ctx: RequestContex, query: String) -> AppResult<RoleQueryResult>;
}
