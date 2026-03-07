use domain::{DomainEventId, Event, UserId, events::DomainEvent};

use crate::{SubjectContex, error::AppResult};



#[async_trait::async_trait]
pub trait EventRepository <T: Clone + Event>{
    async fn get_by_user(&self, ctx: SubjectContex, user_id: UserId) -> AppResult<DomainEvent<T>>;
    async fn get_by_id(&self, ctx: SubjectContex, id: DomainEventId) -> AppResult<DomainEvent<T>>;
    async fn get_by_table(&self, ctx: SubjectContex, table: &str) -> AppResult<Vec<DomainEvent<T>>>;
    async fn get_users_paginated(&self, ctx: SubjectContex,sort_by: &str, page: u32, page_size: u32) -> AppResult<Vec<DomainEvent<T>>>;
}
