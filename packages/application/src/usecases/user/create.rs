use std::sync::Arc;

use domain::User;

use crate::{ SubjectContex, authorization::ports::AuthorizationService, dto::user_dto::{input::CreateUserInput, output::OwnerUserOutput}, error::AppResult, ports::UserRepository};


pub struct CreateUserUseCase
{
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl CreateUserUseCase {
    pub async fn execute(&self, ctx: SubjectContex, input: CreateUserInput) -> AppResult<OwnerUserOutput> {
        let user: User = self.repo.create(ctx, input.try_into()?).await?;
        Ok(OwnerUserOutput::from(user))
    }
}

