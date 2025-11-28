use domain::UserId;

use crate::{error::ApplicationError, ports::UserRepository};


pub struct DeleteUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> DeleteUserUseCase<R> {
    pub async fn execute(&self, user_id: UserId) -> Result<(), ApplicationError> {
        self.repo.delete(&user_id).await?;
        Ok(())
    }
}
