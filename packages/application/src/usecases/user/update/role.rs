use domain::{RoleId, UserId};

use crate::{error::ApplicationError, ports::UserRepository};


pub struct AssignRoleToUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> AssignRoleToUserUseCase<R> {
    pub async fn execute(&self, user_id: UserId , role_id:RoleId) -> Result<(), ApplicationError> {
        self.repo.assign_role(user_id, role_id).await?;
        Ok(())
    }
}
