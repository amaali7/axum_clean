use domain::Role;

use crate::{ dto::role_dto::{input::UpdateRoleInput, output::PrivilegeRoleOutput}, error::AppResult, ports::RoleRepository};


pub struct UpdateRoleUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> UpdateRoleUseCase<R> {
    pub async fn execute(&self, input: UpdateRoleInput) -> AppResult<PrivilegeRoleOutput> {
        let role = Role::try_from(input)?;
        self.repo.update(role.clone()).await?;
        Ok(PrivilegeRoleOutput::from(role))
    }
}
