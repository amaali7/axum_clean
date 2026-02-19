use domain::User;

use crate::{ RequestContex, dto::user_dto::{input::UpdateUserInput, output::OwnerUserOutput}, error::AppResult, ports::UserRepository};


pub struct UpdateUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UpdateUserUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, input: UpdateUserInput) -> AppResult<OwnerUserOutput> {
        let user: User = self.repo.update( ctx, input.try_into()?).await?;
        Ok(OwnerUserOutput::from(user))
    }
}
