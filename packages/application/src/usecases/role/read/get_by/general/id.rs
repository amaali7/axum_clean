use domain::RoleId;

use crate::{RequestContex, dto::role_dto::output::GeneralRoleOutput, error::AppResult, ports::RoleRepository};

pub struct GetRoleByIdGenaralUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> GetRoleByIdGenaralUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, id: RoleId) -> AppResult<GeneralRoleOutput> {
        Ok(self.repo.get_by_id( ctx,id.clone()).await?.into())
        
    }
}
