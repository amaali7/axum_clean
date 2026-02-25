use crate::error::{InterfaceError, InterfaceResult};
use chrono::{DateTime, TimeZone, Utc};
use domain::DateTime as DomainDateTime;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InterfaceDateTime(DateTime<Utc>);

impl InterfaceDateTime {
    pub fn new(datetime: DomainDateTime) -> InterfaceResult<Self> {
        match Utc.timestamp_opt(datetime.datetime(), 0).single() {
            Some(datetime) => Ok(Self(datetime)),
            None => Err(InterfaceError::InvalidTimestamp),
        }
    }

    pub fn datetime(&self) -> DateTime<Utc> {
        self.0.clone()
    }
}

impl Deref for InterfaceDateTime {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InterfaceDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InterfaceDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<InterfaceDateTime> for DomainDateTime {
    fn try_from(value: InterfaceDateTime) -> InterfaceResult<Self> {
        Ok(Self::new(value.datetime().timestamp()))
    }

    type Error = InterfaceError;
}

impl TryFrom<DomainDateTime> for InterfaceDateTime {
    fn try_from(value: DomainDateTime) -> InterfaceResult<Self> {
        Self::new(value)
    }

    type Error = InterfaceError;
}
