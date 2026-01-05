use domain::{Permission, UserId};

use crate::{error::AppResult, ports::UserRepository};


pub struct RemovePermissionFromUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> RemovePermissionFromUserUseCase<R> {
    pub async fn execute(&self, user_id: UserId , permission:Permission) -> AppResult<bool> {
        self.repo.remove_permission(user_id, permission).await
    }
}
