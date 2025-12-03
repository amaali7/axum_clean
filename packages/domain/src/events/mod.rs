pub mod report;
pub mod role;
pub mod user;

use std::ops::{Deref, DerefMut};

pub use report::ReportEvent;
pub use role::RoleEvent;
pub use user::UserEvent;

// pub trait DomainEvent: std::fmt::Debug + Send + Sync {
//     fn event_type(&self) -> &'static str;
//     fn occurred_at(&self) -> String;
//     fn version(&self) -> u64 {
//         1
//     }
// }

#[derive(Debug, Clone)]
pub enum EventType {
    User(UserEvent),
    Role(RoleEvent),
    Report(ReportEvent),
}

#[derive(Debug, Clone)]
pub struct DomainEventId(String);
impl DomainEventId {
    pub fn new() -> Self {
        Self(String::new())
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
