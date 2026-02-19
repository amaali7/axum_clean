use std::ops::{Deref, DerefMut};

use crate::{DateTime, UserId};

#[derive(Debug, Clone)]
pub enum Table {
    User,
    Role,
    Report,
}

#[derive(Debug, Clone)]
pub struct DomainEventId(String);
impl DomainEventId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
    pub fn id(&self) -> String {
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for DomainEventId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DomainEventId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait Event: Clone {
    fn get_type(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct DomainEvent<T: Clone + Event> {
    id: DomainEventId,
    table: Table,
    action: String,
    user_id: UserId,
    occurred_at: DateTime,
    before: T,
    after: T,
}

impl<T: Clone + Event> DomainEvent<T> {
    pub fn new(
        id: &str,
        table: Table,
        action: String,
        user_id: UserId,
        occurred_at: DateTime,
        before: T,
        after: T,
    ) -> Self {
        Self {
            id: DomainEventId::new(id),
            occurred_at,
            action,
            table,
            user_id,
            before,
            after,
        }
    }
    pub fn id(&self) -> DomainEventId {
        self.id.clone()
    }
    pub fn table(&self) -> Table {
        self.table.clone()
    }
    pub fn action(&self) -> String {
        self.action.clone()
    }
    pub fn user_id(&self) -> UserId {
        self.user_id.clone()
    }
    pub fn occurred_at(&self) -> DateTime {
        self.occurred_at.clone()
    }
    pub fn before(&self) -> T {
        self.before.clone()
    }
    pub fn after(&self) -> T {
        self.after.clone()
    }
}
