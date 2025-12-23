use domain::Username;

use crate::{dto::user_dto::output::PrivilegeUserOutput, error::{AppResult, ApplicationError}, ports::UserRepository};

pub struct GetUserByUsernamePrivilegeUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByUsernamePrivilegeUseCase<R> {
    pub async fn execute(&self, username: Username) -> AppResult<PrivilegeUserOutput> {
        let result = self.repo.get_by_username(username.clone()).await?;
        match result {
            Some(user) => Ok(PrivilegeUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", username))),
        }
    }
}
