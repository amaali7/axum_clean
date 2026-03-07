use domain::Name;

use crate::{SubjectContex, dto::role_dto::output::PrivilegeRoleOutput, error::AppResult, ports::RoleRepository};


pub struct GetRoleByNamePrivilegeUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> GetRoleByNamePrivilegeUseCase<R> {
    pub async fn execute(&self, ctx: SubjectContex, name: Name) -> AppResult<PrivilegeRoleOutput> {
        Ok(self.repo.get_by_name(ctx, name.clone()).await?.into())
    }
}
