use domain::User;

use crate::{ dto::user_dto::{input::CreateUserInput, output::OwnerUserOutput}, error::AppResult, ports::UserRepository};


pub struct CreateUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub async fn execute(&self, input: CreateUserInput) -> AppResult<OwnerUserOutput> {
       
        let user_builder = User::new(input.id);
                    let user = user_builder.build()?;

        self.repo.save(user.clone()).await?;
        Ok(OwnerUserOutput::from(user))
    }
}

