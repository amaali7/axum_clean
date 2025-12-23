use domain::Username;

use crate::{dto::user_dto::output::OwnerUserOutput, error::{AppResult, ApplicationError}, ports::UserRepository};


pub struct GetUserByUsernameOwnerUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByUsernameOwnerUseCase<R> {
    pub async fn execute(&self, username: Username) -> AppResult<OwnerUserOutput> {
        let result = self.repo.get_by_username(username.clone()).await?;
        match result {
            Some(user) => Ok(OwnerUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", username))),
        }
    }
}

