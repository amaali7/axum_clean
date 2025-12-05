use domain::Role;
use crate::{dto::role_dto::{input::CreateRoleInput, output::PrivilegeRoleOutput}, error::AppResult, ports::RoleRepository};


pub struct CreateRoleUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> CreateRoleUseCase<R> {
    pub async fn execute(&self, input: CreateRoleInput) -> AppResult<PrivilegeRoleOutput> {
        let role = Role::try_from(input)?;
        self.repo.save(&role).await?;
        Ok(PrivilegeRoleOutput::from(role))
    }
}
