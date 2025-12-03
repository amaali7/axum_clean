
use domain::Email;

use crate::{dto::user_dto::output::GeneralUserOutput, error::{AppResult, ApplicationError}, ports::UserRepository};


pub struct GetUserByEmailGeneralUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByEmailGeneralUseCase<R> {
    pub async fn execute(&self, email: Email ) -> AppResult<GeneralUserOutput> {
        let result = self.repo.get_by_email(&email).await?;
        match result {
            Some(user) => Ok(GeneralUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", email))),
        }
    }
}
