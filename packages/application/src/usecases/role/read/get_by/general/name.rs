use domain::Name;

use crate::{ SubjectContex, dto::role_dto::output::GeneralRoleOutput, error::AppResult, ports::RoleRepository};


pub struct GetRoleByNameGeneralUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> GetRoleByNameGeneralUseCase<R> {
    pub async fn execute(&self, ctx: SubjectContex, name: Name) -> AppResult<GeneralRoleOutput> {
        Ok(self.repo.get_by_name(ctx, name.clone()).await?.into())
    }
}
