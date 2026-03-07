
use std::sync::Arc;

use domain::UserId;

use crate::{SubjectContex, authorization::ports::AuthorizationService, dto::user_dto::output::GeneralUserOutput, error::AppResult, ports::UserRepository};


pub struct GetUserByIdGenaralUseCase {
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl GetUserByIdGenaralUseCase {
    pub async fn execute(&self, ctx: SubjectContex, id: UserId) -> AppResult<GeneralUserOutput> {
            Ok(self.repo.get_by_id(ctx, id.clone()).await?.into())
    }
}
