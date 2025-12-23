use domain::Email;

use crate::{ dto::user_dto::output::PrivilegeUserOutput, error::{AppResult, ApplicationError}, ports::UserRepository};

pub struct GetUserByEmailPrivilegeUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByEmailPrivilegeUseCase<R> {
    pub async fn execute(&self, email: Email ) -> AppResult<PrivilegeUserOutput> {
        let result = self.repo.get_by_email(email.clone()).await?;
        match result {
            Some(user) => Ok(PrivilegeUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", email))),
        }
    }
}
