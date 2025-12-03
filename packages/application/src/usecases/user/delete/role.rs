use domain::{RoleId, UserId};

use crate::{error::AppResult, ports::UserRepository};


pub struct RemoveRoleFromUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> RemoveRoleFromUserUseCase<R> {
    pub async fn execute(&self, user_id: UserId , role_id:RoleId) -> AppResult<()> {
        self.repo.remove_role(user_id, role_id).await
    }
}
