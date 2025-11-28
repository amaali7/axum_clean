use domain::{RoleId, UserId};

use crate::{error::ApplicationError, ports::UserRepository};


pub struct RemoveRoleToUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> RemoveRoleToUserUseCase<R> {
    pub async fn execute(&self, user_id: UserId , role_id:RoleId) -> Result<(), ApplicationError> {
        self.repo.remove_role(user_id, role_id).await?;
        Ok(())
    }
}
