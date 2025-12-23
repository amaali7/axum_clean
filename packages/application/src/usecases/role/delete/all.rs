
use domain::RoleId;

use crate::{error::AppResult, ports::RoleRepository};


pub struct DeleteRoleUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> DeleteRoleUseCase<R> {
    pub async fn execute(&self, role_id: RoleId) -> AppResult<()> {
        self.repo.delete(role_id.clone()).await
    }
}
