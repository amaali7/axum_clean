use domain::Name;

use crate::{ dto::role_dto::output::GeneralRoleOutput, error::{AppResult, ApplicationError}, ports::RoleRepository};


pub struct GetRoleByNameGeneralUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> GetRoleByNameGeneralUseCase<R> {
    pub async fn execute(&self, name: Name) -> AppResult<GeneralRoleOutput> {
        let result = self.repo.get_by_name(&name).await?;
        match result {
            Some(role) => Ok(GeneralRoleOutput::from(role)),
            None => Err(ApplicationError::Repository(format!("Role : {} not found", name))),
        }
    }
}
