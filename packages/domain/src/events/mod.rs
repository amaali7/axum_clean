pub mod report;
pub mod role;
pub mod user;

use std::ops::{Deref, DerefMut};

pub use report::ReportEvent;
pub use role::RoleEvent;
pub use user::UserEvent;

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
}
