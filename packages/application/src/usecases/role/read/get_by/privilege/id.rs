
use domain::UserId;

use crate::{dto::user_dto::output::PrivilegeUserOutput, error::{AppResult, ApplicationError}, ports::UserRepository};


pub struct GetRoleByIdPrivilegeUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetRoleByIdPrivilegeUseCase<R> {
    pub async fn execute(&self, id: UserId) -> AppResult<PrivilegeUserOutput> {
        let result = self.repo.get_by_id(id.clone()).await?;
        match result {
            Some(user) => Ok(PrivilegeUserOutput::from(user)),
            None => Err(ApplicationError::Repository(format!("User : {} not found", id))),
        }
    }
}
