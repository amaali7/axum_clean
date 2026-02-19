use domain::UserId;

use crate::{RequestContex, error::AppResult, ports::UserRepository};


pub struct DeleteUserUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> DeleteUserUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, user_id: UserId) -> AppResult<bool> {
        self.repo.delete(ctx, user_id.clone()).await
    }
}
