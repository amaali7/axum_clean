pub mod report;
pub mod role;
pub mod user;

use std::ops::{Deref, DerefMut};

use domain::events::{DomainEvent, DomainEventId, EventType};
use report::SerializedReportEvent;
use role::SerializedRoleEvent;
use serde::{Deserialize, Serialize};
use user::SerializedUserEvent;

use crate::error::{InfrastructureError, InfrastructureResult};

use super::{value_objects::SerializedDateTime, SerializedUserId};

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

    pub fn as_str(&self) -> &str {
        &self.0
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
    occurred_by: SerializedUserId,
    occurred_at: SerializedDateTime,
}

impl SerializedEvent {
    pub fn new(
        id: &str,
        event_type: SerializedEventType,
        occurred_by: SerializedUserId,
        occurred_at: SerializedDateTime,
    ) -> Self {
        Self {
            id: SerializedEventId::new(id),
            event_type,
            occurred_by,
            occurred_at,
        }
    }
    pub fn id(&self) -> SerializedEventId {
        self.id.clone()
    }
    pub fn event_type(&self) -> SerializedEventType {
        self.event_type.clone()
    }
    pub fn occurred_by(&self) -> SerializedUserId {
        self.occurred_by.clone()
    }
    pub fn occurred_at(&self) -> SerializedDateTime {
        self.occurred_at.clone()
    }
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

impl TryFrom<EventType> for SerializedEventType {
    type Error = InfrastructureError;

    fn try_from(value: EventType) -> InfrastructureResult<Self> {
        Ok(match value {
            EventType::User(user_event) => Self::User(user_event.try_into()?),
            EventType::Role(role_event) => Self::Role(role_event.try_into()?),
            EventType::Report(report_event) => Self::Report(report_event.try_into()?),
        })
    }
}

impl TryFrom<SerializedEventType> for EventType {
    type Error = InfrastructureError;

    fn try_from(value: SerializedEventType) -> InfrastructureResult<Self> {
        Ok(match value {
            SerializedEventType::User(user_event) => Self::User(user_event.try_into()?),
            SerializedEventType::Role(role_event) => Self::Role(role_event.try_into()?),
            SerializedEventType::Report(report_event) => Self::Report(report_event.try_into()?),
        })
    }
}

impl TryFrom<DomainEvent> for SerializedEvent {
    type Error = InfrastructureError;

    fn try_from(value: DomainEvent) -> InfrastructureResult<Self> {
        Ok(Self::new(
            value.id().as_str(),
            value.event_type().try_into()?,
            value.occurred_by().into(),
            value.occurred_at().try_into()?,
        ))
    }
}

impl TryFrom<SerializedEvent> for DomainEvent {
    type Error = InfrastructureError;

    fn try_from(value: SerializedEvent) -> InfrastructureResult<Self> {
        Ok(Self::new(
            value.id().as_str(),
            value.event_type().try_into()?,
            value.occurred_by().into(),
            value.occurred_at().try_into()?,
        ))
    }
}
