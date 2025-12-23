use domain::UserId;

use crate::{error::AppResult, ports::UserRepository};


pub struct DeleteUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> DeleteUserUseCase<R> {
    pub async fn execute(&self, user_id: UserId) -> AppResult<()> {
        self.repo.delete(user_id.clone()).await
    }
}
