use domain::User;

use crate::{ dto::user_dto::{input::UpdateUserInput, output::OwnerUserOutput}, error::AppResult, ports::UserRepository};


pub struct UpdateUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UpdateUserUseCase<R> {
    pub async fn execute(&self, input: UpdateUserInput) -> AppResult<OwnerUserOutput> {
        let user = User::new(input.id).build()?;
        self.repo.update(user.clone()).await?;
        Ok(OwnerUserOutput::from(user))
    }
}
