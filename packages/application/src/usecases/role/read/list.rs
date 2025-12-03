use crate::{dto::role_dto::output::PrivilegeRoleOutput, error::{AppResult, ApplicationError}, ports::RoleRepository};


pub struct ListRoleUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> ListRoleUseCase<R> {
    pub async fn execute(&self) -> AppResult<Vec<PrivilegeRoleOutput>> {
        let result = self.repo.list().await?;
        if !result.is_empty() {
            Err(ApplicationError::Repository("Users not found".to_string()))
        } else {
            let users: Vec<PrivilegeRoleOutput> = result.into_iter().map(|user| PrivilegeRoleOutput::from(user)).collect();
            Ok(users)
        }
    }
}
