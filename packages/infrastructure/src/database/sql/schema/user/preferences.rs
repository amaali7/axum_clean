use crate::surreal::sql::schema::{FieldPath, PathSegment, QueryField};

#[derive(Debug, Clone)]
pub enum UserPreferencesField {
    EmailNotifications,
    PushNotifications,
    TwoFactorAuth,
    Language,
}

impl QueryField for UserPreferencesField {
    fn table_ref(&self) -> &str {
        "user"
    }

    fn path(&self) -> FieldPath {
        vec![PathSegment::Field(
            match self {
                Self::EmailNotifications => "email_notfications",
                Self::PushNotifications => "push_notifications",
                Self::TwoFactorAuth => "two_factor_auth",
                Self::Language => "language",
            }
            .into(),
        )]
    }
}

/// user.profile
pub struct PreferencesPath {
    base: FieldPath,
}

impl UserField {
    pub fn preferences(self) -> PreferencesPath {
        PreferencesPath { base: self.path() }
    }
}
