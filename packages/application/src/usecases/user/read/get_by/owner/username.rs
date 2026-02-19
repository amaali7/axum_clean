use domain::Username;

use crate::{RequestContex, dto::user_dto::output::OwnerUserOutput, error::AppResult, ports::UserRepository};


pub struct GetUserByUsernameOwnerUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByUsernameOwnerUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, username: Username) -> AppResult<OwnerUserOutput> {
        Ok(self.repo.get_by_username(ctx, username.clone()).await?.into())
    }
}

