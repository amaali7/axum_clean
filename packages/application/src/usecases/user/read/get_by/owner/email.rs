
use std::sync::Arc;

use domain::Email;

use crate::{SubjectContex, authorization::ports::AuthorizationService, dto::user_dto::output::OwnerUserOutput, error::AppResult, ports::UserRepository};


pub struct GetUserByEmailOwnerUseCase {
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl GetUserByEmailOwnerUseCase {
    pub async fn execute(&self, ctx: SubjectContex, email: Email ) -> AppResult<OwnerUserOutput> {
        Ok(self.repo.get_by_email(ctx, email.clone()).await?.into())
    }
}

