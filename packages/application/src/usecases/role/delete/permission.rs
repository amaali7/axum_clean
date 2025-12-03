use domain::{Permission, RoleId};

use crate::{error::AppResult, ports::RoleRepository};


pub struct RemovePermissionFromRoleUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> RemovePermissionFromRoleUseCase<R> {
    pub async fn execute(&self, role_id: RoleId , permission:Permission) -> AppResult<()> {
        self.repo.remove_permission(role_id, permission).await
    }
}
