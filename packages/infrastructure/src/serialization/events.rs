use std::ops::{Deref, DerefMut};

use domain::{
    events::{DomainEvent, DomainEventId},
    Event, Report, Role, Table, User,
};
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

use super::{
    report::report::SerializedReport, role::SerializedRole, user::user::SerializedUser,
    value_objects::SerializedDateTime, SerializedUserId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerializedTable {
    User,
    Role,
    Report,
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
pub struct SerializedDomainEvent<T: Clone + Event> {
    id: SerializedEventId,
    table: SerializedTable,
    action: String,
    user_id: SerializedUserId,
    occurred_at: SerializedDateTime,
    before: T,
    after: T,
}

impl<T: Clone + Event> SerializedDomainEvent<T> {
    pub fn new(
        id: &str,
        table: SerializedTable,
        action: String,
        user_id: SerializedUserId,
        occurred_at: SerializedDateTime,
        before: T,
        after: T,
    ) -> Self {
        Self {
            id: SerializedEventId::new(id),
            occurred_at,
            action,
            table,
            user_id,
            before,
            after,
        }
    }
    pub fn id(&self) -> SerializedEventId {
        self.id.clone()
    }
    pub fn table(&self) -> SerializedTable {
        self.table.clone()
    }
    pub fn action(&self) -> String {
        self.action.clone()
    }
    pub fn user_id(&self) -> SerializedUserId {
        self.user_id.clone()
    }
    pub fn occurred_at(&self) -> SerializedDateTime {
        self.occurred_at.clone()
    }
    pub fn before(&self) -> T {
        self.before.clone()
    }
    pub fn after(&self) -> T {
        self.after.clone()
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

impl From<Table> for SerializedTable {
    fn from(value: Table) -> Self {
        match value {
            Table::User => SerializedTable::User,
            Table::Role => SerializedTable::Role,
            Table::Report => SerializedTable::Report,
        }
    }
}

impl From<SerializedTable> for Table {
    fn from(value: SerializedTable) -> Self {
        match value {
            SerializedTable::User => Table::User,
            SerializedTable::Role => Table::Role,
            SerializedTable::Report => Table::Report,
        }
    }
}
// Report

impl TryFrom<DomainEvent<Report>> for SerializedDomainEvent<SerializedReport> {
    type Error = InfrastructureError;

    fn try_from(value: DomainEvent<Report>) -> InfrastructureResult<Self> {
        Ok(Self::new(
            value.id().as_str(),
            value.table().into(),
            value.action().into(),
            value.user_id().into(),
            value.occurred_at().try_into()?,
            value.before().try_into()?,
            value.after().try_into()?,
        ))
    }
}

impl TryFrom<SerializedDomainEvent<SerializedReport>> for DomainEvent<Report> {
    type Error = InfrastructureError;

    fn try_from(value: SerializedDomainEvent<SerializedReport>) -> InfrastructureResult<Self> {
        Ok(Self::new(
            value.id().as_str(),
            value.table().into(),
            value.action().into(),
            value.user_id().into(),
            value.occurred_at().try_into()?,
            value.before().try_into()?,
            value.after().try_into()?,
        ))
    }
}

// Role

impl TryFrom<DomainEvent<Role>> for SerializedDomainEvent<SerializedRole> {
    type Error = InfrastructureError;

    fn try_from(value: DomainEvent<Role>) -> InfrastructureResult<Self> {
        Ok(Self::new(
            value.id().as_str(),
            value.table().into(),
            value.action().into(),
            value.user_id().into(),
            value.occurred_at().try_into()?,
            value.before().try_into()?,
            value.after().try_into()?,
        ))
    }
}

impl TryFrom<SerializedDomainEvent<SerializedRole>> for DomainEvent<Role> {
    type Error = InfrastructureError;

    fn try_from(value: SerializedDomainEvent<SerializedRole>) -> InfrastructureResult<Self> {
        Ok(Self::new(
            value.id().as_str(),
            value.table().into(),
            value.action().into(),
            value.user_id().into(),
            value.occurred_at().try_into()?,
            value.before().try_into()?,
            value.after().try_into()?,
        ))
    }
}
// User
impl TryFrom<DomainEvent<User>> for SerializedDomainEvent<SerializedUser> {
    type Error = InfrastructureError;

    fn try_from(value: DomainEvent<User>) -> InfrastructureResult<Self> {
        Ok(Self::new(
            value.id().as_str(),
            value.table().into(),
            value.action().into(),
            value.user_id().into(),
            value.occurred_at().try_into()?,
            value.before().try_into()?,
            value.after().try_into()?,
        ))
    }
}

impl TryFrom<SerializedDomainEvent<SerializedUser>> for DomainEvent<User> {
    type Error = InfrastructureError;

    fn try_from(value: SerializedDomainEvent<SerializedUser>) -> InfrastructureResult<Self> {
        Ok(Self::new(
            value.id().as_str(),
            value.table().into(),
            value.action().into(),
            value.user_id().into(),
            value.occurred_at().try_into()?,
            value.before().try_into()?,
            value.after().try_into()?,
        ))
    }
}
