use domain::Role;
use crate::{RequestContex, dto::role_dto::{input::CreateRoleInput, output::PrivilegeRoleOutput}, error::AppResult, ports::RoleRepository};


pub struct CreateRoleUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> CreateRoleUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, input: CreateRoleInput) -> AppResult<PrivilegeRoleOutput> {
        let role = Role::try_from(input)?;
        self.repo.create(ctx, role.clone()).await?;
        Ok(PrivilegeRoleOutput::from(role))
    }
}
