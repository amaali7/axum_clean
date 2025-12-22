use domain::{events::DomainEventId, Permission};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserRecord {
    id: Option<SerializedUserId>,
    email: String,
    username: String,
    profile: SerializedUserProfile,
    roles: HashSet<SerializedRoleId>,
    permissions: HashSet<Permission>, // Cached permissions for performance
    preferences: SerializedUserPreferences,
    status: UserStatus,
    events: Vec<DomainEventId>,
}

impl User {
    pub fn new(id: UserRecordId) -> UserBuilder {
        UserBuilder::new(id)
    }
    // Basic getters - return references to avoid cloning
    pub fn id(&self) -> SerializedUserId {
        self.id.clone()
    }

    pub fn email(&self) -> Email {
        self.email.clone()
    }

    pub fn username(&self) -> Username {
        self.username.clone()
    }

    pub fn profile(&self) -> UserProfile {
        self.profile.clone()
    }

    pub fn preferences(&self) -> UserPreferences {
        self.preferences.clone()
    }

    pub fn status(&self) -> UserStatus {
        self.status.clone()
    }

    // Collection getters - return references to avoid cloning
    pub fn roles(&self) -> HashSet<SerializedRoleId> {
        self.roles.clone()
    }

    pub fn permissions(&self) -> HashSet<Permission> {
        self.permissions.clone()
    }

    pub fn events(&self) -> Vec<DomainEventId> {
        self.events.clone()
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Suspended,
    Inactive,
    Banned,
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::Inactive
    }
}

// Builder pattern for complex object creation
pub struct UserBuilder {
    id: UserRecordId,
    email: Option<Email>,
    username: Option<Username>,
    profile: Option<UserProfile>,
    roles: HashSet<RoleId>,
    permissions: HashSet<Permission>, // Cached permissions for performance
    events: Vec<DomainEventId>,
    preferences: Option<UserPreferences>,
    status: UserStatus,
}

impl UserBuilder {
    pub fn new(id: UserRecordId) -> Self {
        Self {
            email: None,
            username: None,
            profile: None,
            roles: HashSet::new(),
            permissions: HashSet::new(),
            events: Vec::new(),
            id,
            preferences: None,
            status: UserStatus::Inactive,
        }
    }

    pub fn set_preferences(&mut self, preferences: UserPreferences) -> &mut Self {
        self.preferences = Some(preferences);
        self
    }
    pub fn set_status(&mut self, status: UserStatus) -> &mut Self {
        self.status = status;
        self
    }
    pub fn set_email(&mut self, email: Email) -> &mut Self {
        self.email = Some(email);
        self
    }
    pub fn add_roles(&mut self, roles: HashSet<RoleId>) -> &mut Self {
        self.roles.extend(roles);
        self
    }
    pub fn add_permissions(&mut self, permissions: HashSet<Permission>) -> &mut Self {
        self.permissions.extend(permissions);
        self
    }
    pub fn add_role(&mut self, role: RoleId) -> &mut Self {
        self.roles.insert(role);
        self
    }
    pub fn add_permission(&mut self, permission: Permission) -> &mut Self {
        self.permissions.insert(permission);
        self
    }
    pub fn set_username(&mut self, username: Username) -> &mut Self {
        self.username = Some(username);
        self
    }

    pub fn set_profile(&mut self, profile: UserProfile) -> &mut Self {
        self.profile = Some(profile);
        self
    }
    fn add_event(&mut self, event: DomainEventId) -> &mut Self {
        self.events.push(event);
        self
    }

    pub fn build(self) -> DomainResult<User> {
        Ok(User {
            id: self.id,
            email: self
                .email
                .ok_or(DomainError::ValidationError("Email not found".to_string()))?,
            username: self.username.ok_or(DomainError::ValidationError(
                "Username not found".to_string(),
            ))?,
            profile: self.profile.ok_or(DomainError::ValidationError(
                "Profile not found".to_string(),
            ))?,
            roles: self.roles,
            permissions: self.permissions,
            preferences: self.preferences.unwrap_or_default(),
            status: self.status,
            events: self.events,
        })
    }
}
