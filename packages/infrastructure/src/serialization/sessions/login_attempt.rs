use serde::Serialize;

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::{
            SerializedDateTime, SerializedIpAddress, SerializedToken, SerializedUsername,
        },
        SerializedUserId,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerializedLoginAttemptId(String);

impl SerializedLoginAttemptId {
    // Create a new SerializedReportId with a UUID
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
impl Serialize for SerializedLoginAttemptId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Debug, Clone)]
pub struct SerializedLoginAttempt {
    id: SerializedLoginAttemptId,
    username: SerializedUsername,
    ip_address: SerializedIpAddress,
    created_at: SerializedDateTime,
    success: bool,
}

impl SerializedLoginAttempt {
    pub fn new(id: &str) -> SerializedLoginAttemptBuilder {
        SerializedLoginAttemptBuilder::new(id)
    }

    pub fn id(&self) -> SerializedLoginAttemptId {
        self.id.clone()
    }

    pub fn username(&self) -> SerializedUsername {
        self.username.clone()
    }
    pub fn ip_address(&self) -> SerializedIpAddress {
        self.ip_address.clone()
    }

    pub fn created_at(&self) -> SerializedDateTime {
        self.created_at.clone()
    }

    pub fn success(&self) -> bool {
        self.success.clone()
    }
}

#[derive(Debug, Clone)]
pub struct SerializedLoginAttemptBuilder {
    id: SerializedLoginAttemptId,
    username: Option<SerializedUsername>,
    ip_address: Option<SerializedIpAddress>,
    created_at: Option<SerializedDateTime>,
    success: bool,
}

impl SerializedLoginAttemptBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: SerializedLoginAttemptId::new(id),
            username: None,
            ip_address: None,
            created_at: None,
            success: false,
        }
    }

    pub fn set_username(&mut self, username: SerializedUsername) -> &mut Self {
        self.username = Some(username);
        self
    }
    pub fn set_ip_address(&mut self, ip_address: SerializedIpAddress) -> &mut Self {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn set_created_at(&mut self, created_at: SerializedDateTime) -> &mut Self {
        self.created_at = Some(created_at);
        self
    }

    pub fn set_success(&mut self, success: bool) -> &mut Self {
        self.success = success;
        self
    }

    pub fn build(self) -> InfrastructureResult<SerializedLoginAttempt> {
        Ok(SerializedLoginAttempt {
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
