
use std::sync::Arc;

use domain::Email;

use crate::{SubjectContex, authorization::ports::AuthorizationService, dto::user_dto::output::GeneralUserOutput, error::AppResult, ports::UserRepository};


pub struct GetUserByEmailGeneralUseCase {
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl GetUserByEmailGeneralUseCase {
    pub async fn execute(&self, ctx: SubjectContex, email: Email ) -> AppResult<GeneralUserOutput> {
        Ok(self.repo.get_by_email(ctx, email.clone()).await?.into())
    }
}
