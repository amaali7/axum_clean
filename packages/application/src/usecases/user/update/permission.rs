use domain::{Permission, UserId};

use crate::{error::ApplicationError, ports::UserRepository};


pub struct AssignPermissionToUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> AssignPermissionToUserUseCase<R> {
    pub async fn execute(&self, user_id: UserId , permission:Permission) -> Result<(), ApplicationError> {
        self.repo.assign_permission(user_id, permission).await?;
        Ok(())
    }
}
