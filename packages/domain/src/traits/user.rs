use async_trait::async_trait;

use crate::{DomainError, Email, RoleId, User, UserId, Username};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<User, DomainError>;
    async fn find_by_email(&self, email: &Email) -> Result<User, DomainError>;
    async fn find_by_username(&self, username: &Username) -> Result<User, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn delete(&self, id: &UserId) -> Result<(), DomainError>;
    async fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError>;
    async fn exists_by_username(&self, username: &Username) -> Result<bool, DomainError>;
    async fn find_users_by_role(&self, role_id: &RoleId) -> Result<Vec<User>, DomainError>;
}

#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_welcome_email(&self, email: &Email, username: &Username) -> Result<(), DomainError>;
    async fn send_email_changed_notification(&self, email: &Email) -> Result<(), DomainError>;
}
