use std::sync::Arc;

use domain::UserId;

use crate::{SubjectContex, authorization::ports::AuthorizationService, error::AppResult, ports::UserRepository};


pub struct DeleteUserUseCase {
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl DeleteUserUseCase {
    pub async fn execute(&self, ctx: SubjectContex, user_id: UserId) -> AppResult<bool> {
        self.repo.delete(ctx, user_id.clone()).await
    }
}
