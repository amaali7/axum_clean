use std::sync::Arc;

use domain::User;

use crate::{ SubjectContex, authorization::ports::AuthorizationService, dto::user_dto::{input::UpdateUserInput, output::OwnerUserOutput}, error::AppResult, ports::UserRepository};


pub struct UpdateUserUseCase{
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl UpdateUserUseCase {
    pub async fn execute(&self, ctx: SubjectContex, input: UpdateUserInput) -> AppResult<OwnerUserOutput> {
        let user: User = self.repo.update( ctx, input.try_into()?).await?;
        Ok(OwnerUserOutput::from(user))
    }
}
