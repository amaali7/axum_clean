use domain::{DomainEventId, Event, UserId, events::DomainEvent};

use crate::{RequestContex, error::AppResult};



#[async_trait::async_trait]
pub trait EventRepository <T: Clone + Event>{
    async fn get_by_user(&self, ctx: RequestContex, user_id: UserId) -> AppResult<DomainEvent<T>>;
    async fn get_by_id(&self, ctx: RequestContex, id: DomainEventId) -> AppResult<DomainEvent<T>>;
    async fn get_by_table(&self, ctx: RequestContex, table: &str) -> AppResult<Vec<DomainEvent<T>>>;
    async fn get_users_paginated(&self, ctx: RequestContex,sort_by: &str, page: u32, page_size: u32) -> AppResult<Vec<DomainEvent<T>>>;
}
