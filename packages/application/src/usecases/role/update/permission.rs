use domain::{Permission, RoleId};

use crate::{error::AppResult, ports::RoleRepository};



pub struct AssignPermissionForRoleUseCase<R: RoleRepository> {
    repo: R,
}

impl<R: RoleRepository> AssignPermissionForRoleUseCase<R> {
    pub async fn execute(&self , role_id:RoleId, permission: Permission) -> AppResult<()> {
        self.repo.assign_permission( role_id, permission).await
    }
}
