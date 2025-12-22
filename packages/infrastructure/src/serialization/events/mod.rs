pub mod report;
pub mod role;
pub mod user;

use std::ops::{Deref, DerefMut};

use domain::events::DomainEventId;
use report::SerializedReportEvent;
use role::SerializedRoleEvent;
use serde::{Deserialize, Serialize};
use user::SerializedUserEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedEventType {
    User(SerializedUserEvent),
    Role(SerializedRoleEvent),
    Report(SerializedReportEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedEventId(String);
impl SerializedEventId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
    pub fn id(&self) -> String {
        self.0.clone()
    }
}

impl Deref for SerializedEventId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializedEventId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedEvent {
    id: SerializedEventId,
    event_type: SerializedEventType,
}

impl From<DomainEventId> for SerializedEventId {
    fn from(value: DomainEventId) -> Self {
        Self::new(&value.id())
    }
}

impl From<SerializedEventId> for DomainEventId {
    fn from(value: SerializedEventId) -> Self {
        Self::new(&value.id())
    }
}
