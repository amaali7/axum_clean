use crate::{dto::user_dto::output::PrivilegeUserOutput, error::{AppResult, ApplicationError}, ports::UserRepository};


pub struct ListUserPrivilegeUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> ListUserPrivilegeUseCase<R> {
    pub async fn execute(&self) -> AppResult<Vec<PrivilegeUserOutput>> {
        let result = self.repo.list().await?;
        if !result.is_empty() {
            Err(ApplicationError::Repository("Users not found".to_string()))
        } else {
            let users: Vec<PrivilegeUserOutput> = result.into_iter().map(|user| PrivilegeUserOutput::from(user)).collect();
            Ok(users)
        }
    }
}

