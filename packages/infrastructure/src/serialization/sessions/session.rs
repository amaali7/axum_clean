use serde::Serialize;

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::{SerializedDateTime, SerializedToken},
        SerializedUserId,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SerializedSessionId(String);

impl SerializedSessionId {
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
impl Serialize for SerializedSessionId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Debug, Clone)]
pub enum SerializedSessionType {
    Start,
}

#[derive(Debug, Clone)]
pub struct SerializedSession {
    id: SerializedSessionId,
    user: SerializedUserId,
    session_type: SerializedSessionType,
    created_at: SerializedDateTime,
    expires_at: SerializedDateTime,
    refresh_token: Option<SerializedToken>,
    access_token: Option<SerializedToken>,
}

impl SerializedSession {
    pub fn new(id: &str) -> SerializedSessionBuilder {
        SerializedSessionBuilder::new(id)
    }

    pub fn id(&self) -> SerializedSessionId {
        self.id.clone()
    }

    pub fn user(&self) -> SerializedUserId {
        self.user.clone()
    }
    pub fn session_type(&self) -> SerializedSessionType {
        self.session_type.clone()
    }

    pub fn created_at(&self) -> SerializedDateTime {
        self.created_at.clone()
    }
    pub fn expires_at(&self) -> SerializedDateTime {
        self.expires_at.clone()
    }

    pub fn refresh_token(&self) -> Option<SerializedToken> {
        self.refresh_token.clone()
    }
    pub fn access_token(&self) -> Option<SerializedToken> {
        self.access_token.clone()
    }
}

#[derive(Debug, Clone)]
pub struct SerializedSessionBuilder {
    id: SerializedSessionId,
    user: Option<SerializedUserId>,
    session_type: Option<SerializedSessionType>,
    created_at: Option<SerializedDateTime>,
    expires_at: Option<SerializedDateTime>,
    refresh_token: Option<SerializedToken>,
    access_token: Option<SerializedToken>,
}

impl SerializedSessionBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: SerializedSessionId::new(id),
            user: None,
            session_type: None,
            created_at: None,
            expires_at: None,
            refresh_token: None,
            access_token: None,
        }
    }

    pub fn set_user(&mut self, user: SerializedUserId) -> &mut Self {
        self.user = Some(user);
        self
    }
    pub fn set_session_type(&mut self, session_type: SerializedSessionType) -> &mut Self {
        self.session_type = Some(session_type);
        self
    }

    pub fn set_created_at(&mut self, created_at: SerializedDateTime) -> &mut Self {
        self.created_at = Some(created_at);
        self
    }
    pub fn set_expires_at(&mut self, expires_at: SerializedDateTime) -> &mut Self {
        self.expires_at = Some(expires_at);
        self
    }
    pub fn set_refresh_token(&mut self, refresh_token: SerializedToken) -> &mut Self {
        self.refresh_token = Some(refresh_token);
        self
    }
    pub fn set_access_token(&mut self, access_token: SerializedToken) -> &mut Self {
        self.access_token = Some(access_token);
        self
    }

    pub fn build(self) -> InfrastructureResult<SerializedSession> {
        Ok(SerializedSession {
            id: self.id,
            created_at: self.created_at.ok_or(InfrastructureError::ValidationError(
                "Created At not found".to_string(),
            ))?,
            expires_at: self.expires_at.ok_or(InfrastructureError::ValidationError(
                "Expiers At not found".to_string(),
            ))?,
            user: self.user.ok_or(InfrastructureError::ValidationError(
                "User  not found".to_string(),
            ))?,
            session_type: self
                .session_type
                .ok_or(InfrastructureError::ValidationError(
                    "Session type not found".to_string(),
                ))?,
            refresh_token: self.refresh_token,
            access_token: self.access_token,
        })
    }
}
