use crate::error::{InfrastructureError, InfrastructureResult};
use chrono::{DateTime, TimeZone, Utc};
use domain::DateTime as DomainDateTime;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InfrastructureDateTime(DateTime<Utc>);

impl InfrastructureDateTime {
    pub fn new(datetime: DomainDateTime) -> InfrastructureResult<Self> {
        match Utc.timestamp_opt(datetime.datetime(), 0).single() {
            Some(datetime) => Ok(Self(datetime)),
            None => Err(InfrastructureError::InvalidTimestamp),
        }
    }

    pub fn datetime(&self) -> DateTime<Utc> {
        self.0.clone()
    }
}

impl Deref for InfrastructureDateTime {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InfrastructureDateTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for InfrastructureDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<InfrastructureDateTime> for DomainDateTime {
    fn try_from(value: InfrastructureDateTime) -> InfrastructureResult<Self> {
        Ok(Self::new(value.datetime().timestamp()))
    }

    type Error = InfrastructureError;
}

impl TryFrom<DomainDateTime> for InfrastructureDateTime {
    fn try_from(value: DomainDateTime) -> InfrastructureResult<Self> {
        Self::new(value)
    }

    type Error = InfrastructureError;
}
