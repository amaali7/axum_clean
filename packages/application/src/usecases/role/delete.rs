
use domain::RoleId;

use crate::{RequestContex, error::AppResult, ports::RoleRepository};


pub struct DeleteRoleUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> DeleteRoleUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, role_id: RoleId) -> AppResult<bool> {
        self.repo.delete(ctx, role_id.clone()).await
    }
}
