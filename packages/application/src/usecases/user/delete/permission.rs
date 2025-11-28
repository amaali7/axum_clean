use domain::{Permission, UserId};

use crate::{error::ApplicationError, ports::UserRepository};


pub struct RemovePermissionToUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> RemovePermissionToUserUseCase<R> {
    pub async fn execute(&self, user_id: UserId , permission:Permission) -> Result<(), ApplicationError> {
        self.repo.remove_permission(user_id, permission).await?;
        Ok(())
    }
}
