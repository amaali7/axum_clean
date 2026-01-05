pub mod report;
pub mod role;
pub mod user;

use std::ops::{Deref, DerefMut};

pub use report::ReportEvent;
pub use role::RoleEvent;
pub use user::UserEvent;

use crate::{DateTime, UserId};

#[derive(Debug, Clone)]
pub enum EventType {
    User(UserEvent),
    Role(RoleEvent),
    Report(ReportEvent),
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

#[derive(Debug, Clone)]
pub struct DomainEvent {
    id: DomainEventId,
    event_type: EventType,
    occurred_by: UserId,
    occurred_at: DateTime,
}

impl DomainEvent {
    pub fn new(
        id: &str,
        event_type: EventType,
        occurred_by: UserId,
        occurred_at: DateTime,
    ) -> Self {
        Self {
            id: DomainEventId::new(id),
            event_type,
            occurred_by,
            occurred_at,
        }
    }
    pub fn id(&self) -> DomainEventId {
        self.id.clone()
    }
    pub fn event_type(&self) -> EventType {
        self.event_type.clone()
    }
    pub fn occurred_by(&self) -> UserId {
        self.occurred_by.clone()
    }
    pub fn occurred_at(&self) -> DateTime {
        self.occurred_at.clone()
    }
}
