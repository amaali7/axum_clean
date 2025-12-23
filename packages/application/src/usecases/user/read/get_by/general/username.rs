use domain::Username;

use crate::{dto::user_dto::output::GeneralUserOutput, error::{AppResult, ApplicationError}, ports::UserRepository};

pub struct GetUserByUsernameGeneralUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByUsernameGeneralUseCase<R> {
    pub async fn execute(&self, username: Username) -> AppResult<GeneralUserOutput> {
        let result = self.repo.get_by_username(username.clone()).await?;
        match result {
            Some(user) => Ok(GeneralUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", username))),
        }
    }
}
