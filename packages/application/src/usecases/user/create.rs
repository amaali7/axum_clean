use std::sync::Arc;


use crate::{ SubjectContex, authorization::ports::AuthorizationService, dto::{user::{command::UserCommand, view::UserView}, user_dto::{input::CreateUserInput, output::OwnerUserOutput}}, error::AppResult, ports::UserRepository};


pub struct CreateUserUseCase
{
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl CreateUserUseCase {
    pub async fn execute(&self, ctx: SubjectContex, input: UserCommand) -> AppResult<UserView> {
        self.repo.create(ctx, input).await
    }
}

