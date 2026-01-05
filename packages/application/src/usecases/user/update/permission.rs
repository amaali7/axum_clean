use domain::{Permission, UserId};

use crate::{error::AppResult, ports::UserRepository};


pub struct AssignPermissionToUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> AssignPermissionToUserUseCase<R> {
    pub async fn execute(&self, user_id: UserId , permission:Permission) -> AppResult<bool> {
        self.repo.assign_permission(user_id, permission).await
    }
}
