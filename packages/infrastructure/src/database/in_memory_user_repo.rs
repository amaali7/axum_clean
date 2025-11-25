use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use domain::user::{UserRepository, UserId, User, UserError};
use async_trait::async_trait;

/// Simple in-memory repository for demonstration/testing.
/// Uses tokio RwLock and an Arc so it can be shared between threads/handlers.
#[derive(Clone, Default)]
pub struct InMemoryUserRepository {
    inner: Arc<RwLock<HashMap<String, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self { inner: Arc::new(RwLock::new(HashMap::new())) }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: User) -> Result<(), UserError> {
        let mut map = self.inner.write().await;
        map.insert(user.id().to_string(), user);
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<User, UserError> {
        let map = self.inner.read().await;
        map.get(&id.to_string()).cloned().ok_or(UserError::NotFound)
    }
}
