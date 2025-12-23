use domain::Name;

use crate::{dto::role_dto::output::PrivilegeRoleOutput, error::{AppResult, ApplicationError}, ports::RoleRepository};


pub struct GetRoleByNamePrivilegeUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> GetRoleByNamePrivilegeUseCase<R> {
    pub async fn execute(&self, name: Name) -> AppResult<PrivilegeRoleOutput> {
        let result = self.repo.get_by_name(name.clone()).await?;
        match result {
            Some(role) => Ok(PrivilegeRoleOutput::from(role)),
            None => Err(ApplicationError::Repository(format!("Role : {} not found", name))),
        }
    }
}
