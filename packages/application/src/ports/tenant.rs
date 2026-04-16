use domain::TenantId;

use crate::{SubjectContex, dto::tenant::{command::TenantCommand, view::TenantView}, error::AppResult};

use super::{ SortBy};

#[derive(Debug, Clone)]
pub enum TenantQueryResult {
    Single(TenantView),
    Array(Vec<TenantView>),
    None,
}

impl TenantQueryResult {
    pub fn get_array(&self) -> Option<Vec<TenantView>> {
        match self {
            TenantQueryResult::Array(users) => Some(users.to_vec()),
            _ => None,
        }
    }
    pub fn get_value(&self) -> Option<TenantView> {
        match self {
            TenantQueryResult::Single(user) => Some(user.clone()),
            _ => None,
        }
    }
}

#[async_trait::async_trait]
pub trait TenantRepository {
    async fn create(&self,ctx: SubjectContex, tenant: TenantCommand) -> AppResult<TenantView>;
    async fn update(&self,ctx: SubjectContex, tenant: TenantCommand) -> AppResult<TenantView>;
    async fn get_by_id(&self,ctx: SubjectContex, id: TenantId) -> AppResult<TenantView>;
    async fn delete(&self,ctx: SubjectContex, id: TenantId) -> AppResult<bool>;
    async fn get_tenants_paginated(&self,ctx: SubjectContex,sort_by: &[SortBy], page: u32, page_size: u32) -> AppResult<Vec<TenantView>>;
    async fn raw_query(&self,ctx: SubjectContex, query: String) -> AppResult<TenantQueryResult>;
}
