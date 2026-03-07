use std::sync::Arc;

use domain::Username;

use crate::{SubjectContex, authorization::ports::AuthorizationService, dto::user_dto::output::GeneralUserOutput, error::AppResult, ports::UserRepository};

pub struct GetUserByUsernameGeneralUseCase {
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl GetUserByUsernameGeneralUseCase {
    pub async fn execute(&self, ctx: SubjectContex, username: Username) -> AppResult<GeneralUserOutput> {
        Ok(self.repo.get_by_username(ctx, username.clone()).await?.into())
    }
}
