use domain::{user::UserPreferences, value_objects::Language};
// serializers/user_preferences_serializer.rs
use serde::{Deserialize, Serialize};

use crate::error::InfrastructureError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealUserPreferences {
    pub email_notifications: bool,
    pub push_notifications: bool,
    pub two_factor_auth: bool,
    pub language: String,
}

impl From<UserPreferences> for SurrealUserPreferences {
    fn from(prefs: UserPreferences) -> Self {
        Self {
            email_notifications: prefs.email_notifications(),
            push_notifications: prefs.push_notifications(),
            two_factor_auth: prefs.two_factor_auth(),
            language: prefs.language().to_string(),
        }
    }
}

impl TryFrom<SurrealUserPreferences> for UserPreferences {
    type Error = InfrastructureError;

    fn try_from(value: SurrealUserPreferences) -> Result<Self, Self::Error> {
        Ok(UserPreferences::new(
            value.email_notifications,
            value.push_notifications,
            value.two_factor_auth,
            Language::new(&value.language)?,
        ))
    }
}
