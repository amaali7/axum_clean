use std::ops::{Deref, DerefMut};

use domain::{
    events::{DomainEvent, DomainEventId},
    Event, Report, Role, Table, User,
};
use serde::{Deserialize, Serialize};

use crate::error::{InfrastructureError, InfrastructureResult};

use super::{
    report::report::InfrastructureReport, role::InfrastructureRole, user::user::InfrastructureUser,
    value_objects::InfrastructureDateTime, InfrastructureUserId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InfrastructureTable {
    User,
    Role,
    Report,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureEventId(String);
impl InfrastructureEventId {
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

impl Deref for InfrastructureEventId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InfrastructureEventId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureDomainEvent<T: Clone + Event> {
    id: InfrastructureEventId,
    table: InfrastructureTable,
    action: String,
    user_id: InfrastructureUserId,
    occurred_at: InfrastructureDateTime,
    before: T,
    after: T,
}

impl<T: Clone + Event> InfrastructureDomainEvent<T> {
    pub fn new(
        id: &str,
        table: InfrastructureTable,
        action: String,
        user_id: InfrastructureUserId,
        occurred_at: InfrastructureDateTime,
        before: T,
        after: T,
    ) -> Self {
        Self {
            id: InfrastructureEventId::new(id),
            occurred_at,
            action,
            table,
            user_id,
            before,
            after,
        }
    }
    pub fn id(&self) -> InfrastructureEventId {
        self.id.clone()
    }
    pub fn table(&self) -> InfrastructureTable {
        self.table.clone()
    }
    pub fn action(&self) -> String {
        self.action.clone()
    }
    pub fn user_id(&self) -> InfrastructureUserId {
        self.user_id.clone()
    }
    pub fn occurred_at(&self) -> InfrastructureDateTime {
        self.occurred_at.clone()
    }
    pub fn before(&self) -> T {
        self.before.clone()
    }
    pub fn after(&self) -> T {
        self.after.clone()
    }
}
impl From<DomainEventId> for InfrastructureEventId {
    fn from(value: DomainEventId) -> Self {
        Self::new(&value.id())
    }
}

impl From<InfrastructureEventId> for DomainEventId {
    fn from(value: InfrastructureEventId) -> Self {
        Self::new(&value.id())
    }
}

impl From<Table> for InfrastructureTable {
    fn from(value: Table) -> Self {
        match value {
            Table::User => InfrastructureTable::User,
            Table::Role => InfrastructureTable::Role,
            Table::Report => InfrastructureTable::Report,
        }
    }
}

impl From<InfrastructureTable> for Table {
    fn from(value: InfrastructureTable) -> Self {
        match value {
            InfrastructureTable::User => Table::User,
            InfrastructureTable::Role => Table::Role,
            InfrastructureTable::Report => Table::Report,
        }
    }
}
// Report

impl TryFrom<DomainEvent<Report>> for InfrastructureDomainEvent<InfrastructureReport> {
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

impl TryFrom<InfrastructureDomainEvent<InfrastructureReport>> for DomainEvent<Report> {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureDomainEvent<InfrastructureReport>) -> InfrastructureResult<Self> {
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

impl TryFrom<DomainEvent<Role>> for InfrastructureDomainEvent<InfrastructureRole> {
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

impl TryFrom<InfrastructureDomainEvent<InfrastructureRole>> for DomainEvent<Role> {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureDomainEvent<InfrastructureRole>) -> InfrastructureResult<Self> {
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
impl TryFrom<DomainEvent<User>> for InfrastructureDomainEvent<InfrastructureUser> {
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

impl TryFrom<InfrastructureDomainEvent<InfrastructureUser>> for DomainEvent<User> {
    type Error = InfrastructureError;

    fn try_from(value: InfrastructureDomainEvent<InfrastructureUser>) -> InfrastructureResult<Self> {
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
