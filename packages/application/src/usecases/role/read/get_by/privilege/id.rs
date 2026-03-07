
use domain::RoleId;

use crate::{SubjectContex, dto::role_dto::output::PrivilegeRoleOutput, error::AppResult, ports::RoleRepository};


pub struct GetRoleByIdPrivilegeUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> GetRoleByIdPrivilegeUseCase<R> {
    pub async fn execute(&self, ctx: SubjectContex, id: RoleId) -> AppResult<PrivilegeRoleOutput> {
        Ok(self.repo.get_by_id( ctx,id.clone()).await?.into())
    }
}
