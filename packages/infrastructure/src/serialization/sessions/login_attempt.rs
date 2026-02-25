use serde::Serialize;

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::{
            InfrastructureDateTime, InfrastructureIpAddress, InfrastructureToken, InfrastructureUsername,
        },
        InfrastructureUserId,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfrastructureLoginAttemptId(String);

impl InfrastructureLoginAttemptId {
    // Create a new InfrastructureReportId with a UUID
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }

    // Get the inner String for database operations
    pub fn id(&self) -> String {
        self.0.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Implement Serialize and Deserialize for your type
impl Serialize for InfrastructureLoginAttemptId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Debug, Clone)]
pub struct InfrastructureLoginAttempt {
    id: InfrastructureLoginAttemptId,
    username: InfrastructureUsername,
    ip_address: InfrastructureIpAddress,
    created_at: InfrastructureDateTime,
    success: bool,
}

impl InfrastructureLoginAttempt {
    pub fn new(id: &str) -> InfrastructureLoginAttemptBuilder {
        InfrastructureLoginAttemptBuilder::new(id)
    }

    pub fn id(&self) -> InfrastructureLoginAttemptId {
        self.id.clone()
    }

    pub fn username(&self) -> InfrastructureUsername {
        self.username.clone()
    }
    pub fn ip_address(&self) -> InfrastructureIpAddress {
        self.ip_address.clone()
    }

    pub fn created_at(&self) -> InfrastructureDateTime {
        self.created_at.clone()
    }

    pub fn success(&self) -> bool {
        self.success.clone()
    }
}

#[derive(Debug, Clone)]
pub struct InfrastructureLoginAttemptBuilder {
    id: InfrastructureLoginAttemptId,
    username: Option<InfrastructureUsername>,
    ip_address: Option<InfrastructureIpAddress>,
    created_at: Option<InfrastructureDateTime>,
    success: bool,
}

impl InfrastructureLoginAttemptBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: InfrastructureLoginAttemptId::new(id),
            username: None,
            ip_address: None,
            created_at: None,
            success: false,
        }
    }

    pub fn set_username(&mut self, username: InfrastructureUsername) -> &mut Self {
        self.username = Some(username);
        self
    }
    pub fn set_ip_address(&mut self, ip_address: InfrastructureIpAddress) -> &mut Self {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn set_created_at(&mut self, created_at: InfrastructureDateTime) -> &mut Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn set_success(&mut self, success: bool) -> &mut Self {
        self.success = success;
        self
    }

    pub fn build(self) -> InfrastructureResult<InfrastructureLoginAttempt> {
        Ok(InfrastructureLoginAttempt {
            id: self.id,
            created_at: self.created_at.ok_or(InfrastructureError::ValidationError(
                "Created At not found".to_string(),
            ))?,
            username: self.username.ok_or(InfrastructureError::ValidationError(
                "User  not found".to_string(),
            ))?,
            ip_address: self.ip_address.ok_or(InfrastructureError::ValidationError(
                "LoginAttempt type not found".to_string(),
            ))?,
            success: self.success,
        })
    }
}
