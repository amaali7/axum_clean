use serde::Serialize;

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::{
        value_objects::{InfrastructureDateTime, InfrastructureToken},
        InfrastructureUserId,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfrastructureSessionId(String);

impl InfrastructureSessionId {
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
impl Serialize for InfrastructureSessionId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Debug, Clone)]
pub enum InfrastructureSessionType {
    Start,
}

#[derive(Debug, Clone)]
pub struct InfrastructureSession {
    id: InfrastructureSessionId,
    user: InfrastructureUserId,
    session_type: InfrastructureSessionType,
    created_at: InfrastructureDateTime,
    expires_at: InfrastructureDateTime,
    refresh_token: Option<InfrastructureToken>,
    access_token: Option<InfrastructureToken>,
}

impl InfrastructureSession {
    pub fn new(id: &str) -> InfrastructureSessionBuilder {
        InfrastructureSessionBuilder::new(id)
    }

    pub fn id(&self) -> InfrastructureSessionId {
        self.id.clone()
    }

    pub fn user(&self) -> InfrastructureUserId {
        self.user.clone()
    }
    pub fn session_type(&self) -> InfrastructureSessionType {
        self.session_type.clone()
    }

    pub fn created_at(&self) -> InfrastructureDateTime {
        self.created_at.clone()
    }
    pub fn expires_at(&self) -> InfrastructureDateTime {
        self.expires_at.clone()
    }

    pub fn refresh_token(&self) -> Option<InfrastructureToken> {
        self.refresh_token.clone()
    }
    pub fn access_token(&self) -> Option<InfrastructureToken> {
        self.access_token.clone()
    }
}

#[derive(Debug, Clone)]
pub struct InfrastructureSessionBuilder {
    id: InfrastructureSessionId,
    user: Option<InfrastructureUserId>,
    session_type: Option<InfrastructureSessionType>,
    created_at: Option<InfrastructureDateTime>,
    expires_at: Option<InfrastructureDateTime>,
    refresh_token: Option<InfrastructureToken>,
    access_token: Option<InfrastructureToken>,
}

impl InfrastructureSessionBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: InfrastructureSessionId::new(id),
            user: None,
            session_type: None,
            created_at: None,
            expires_at: None,
            refresh_token: None,
            access_token: None,
        }
    }

    pub fn set_user(&mut self, user: InfrastructureUserId) -> &mut Self {
        self.user = Some(user);
        self
    }
    pub fn set_session_type(&mut self, session_type: InfrastructureSessionType) -> &mut Self {
        self.session_type = Some(session_type);
        self
    }

    pub fn set_created_at(&mut self, created_at: InfrastructureDateTime) -> &mut Self {
        self.created_at = Some(created_at);
        self
    }
    pub fn set_expires_at(&mut self, expires_at: InfrastructureDateTime) -> &mut Self {
        self.expires_at = Some(expires_at);
        self
    }
    pub fn set_refresh_token(&mut self, refresh_token: InfrastructureToken) -> &mut Self {
        self.refresh_token = Some(refresh_token);
        self
    }
    pub fn set_access_token(&mut self, access_token: InfrastructureToken) -> &mut Self {
        self.access_token = Some(access_token);
        self
    }

    pub fn build(self) -> InfrastructureResult<InfrastructureSession> {
        Ok(InfrastructureSession {
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
