use domain::user::UserPreferences;
use serde::{Deserialize, Serialize};

use crate::{
    error::{InfrastructureError, InfrastructureResult},
    serialization::value_objects::SerializedLanguage,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SerializedUserPreferences {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub two_factor_auth: bool,
    pub language: SerializedLanguage,
}

impl SerializedUserPreferences {
    pub fn new(
        email_notifications: bool,
        push_notifications: bool,
        two_factor_auth: bool,
        language: SerializedLanguage,
    ) -> Self {
        Self {
            email_notifications,
            push_notifications,
            two_factor_auth,
            language,
        }
    }
    pub fn email_notifications(&self) -> bool {
        self.email_notifications.clone()
    }

    pub fn push_notifications(&self) -> bool {
        self.push_notifications.clone()
    }

    pub fn two_factor_auth(&self) -> bool {
        self.two_factor_auth.clone()
    }

    pub fn language(&self) -> SerializedLanguage {
        self.language.clone()
    }
}

impl TryFrom<UserPreferences> for SerializedUserPreferences {
    type Error = InfrastructureError;

    fn try_from(prefs: UserPreferences) -> InfrastructureResult<Self> {
        Ok(Self {
            email_notifications: prefs.email_notifications(),
            push_notifications: prefs.push_notifications(),
            two_factor_auth: prefs.two_factor_auth(),
            language: prefs.language().try_into()?,
        })
    }
}

impl TryFrom<SerializedUserPreferences> for UserPreferences {
    type Error = InfrastructureError;

    fn try_from(value: SerializedUserPreferences) -> Result<Self, Self::Error> {
        Ok(UserPreferences::new(
            value.email_notifications,
            value.push_notifications,
            value.two_factor_auth,
            value.language.try_into()?,
        ))
    }
}
