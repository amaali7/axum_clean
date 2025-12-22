use crate::value_objects::Language;

#[derive(Debug, Clone)]
pub struct UserPreferences {
    email_notifications: bool,
    push_notifications: bool,
    two_factor_auth: bool,
    language: Language,
}

impl UserPreferences {
    pub fn new(
        email_notifications: bool,
        push_notifications: bool,
        two_factor_auth: bool,
        language: Language,
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

    pub fn language(&self) -> Language {
        self.language.clone()
    }
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            email_notifications: true,
            push_notifications: true,
            two_factor_auth: false,
            language: Language::new("english").unwrap(),
        }
    }
}
