
use domain::Email;

use crate::{dto::user_dto::output::OwnerUserOutput, error::{AppResult, ApplicationError}, ports::UserRepository};


pub struct GetUserByEmailOwnerUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByEmailOwnerUseCase<R> {
    pub async fn execute(&self, email: Email ) -> AppResult<OwnerUserOutput> {
        let result = self.repo.get_by_email(&email).await?;
        match result {
            Some(user) => Ok(OwnerUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", email))),
        }
    }
}

