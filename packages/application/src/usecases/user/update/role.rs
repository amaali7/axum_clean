use domain::{RoleId, UserId};

use crate::{error::AppResult, ports::UserRepository};


pub struct AssignRoleToUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> AssignRoleToUserUseCase<R> {
    pub async fn execute(&self, user_id: UserId , role_id:RoleId) -> AppResult<()> {
        self.repo.assign_role(user_id, role_id).await
    }
}
