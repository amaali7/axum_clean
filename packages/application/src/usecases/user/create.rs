use domain::User;

use crate::{ RequestContex, dto::user_dto::{input::CreateUserInput, output::OwnerUserOutput}, error::AppResult, ports::UserRepository};


pub struct CreateUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, input: CreateUserInput) -> AppResult<OwnerUserOutput> {
        let user: User = self.repo.create(ctx, input.try_into()?).await?;
        Ok(OwnerUserOutput::from(user))
    }
}

