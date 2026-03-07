use domain::Username;

use crate::{SubjectContex, dto::user_dto::output::OwnerUserOutput, error::AppResult, ports::UserRepository};


pub struct GetUserByUsernameOwnerUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByUsernameOwnerUseCase<R> {
    pub async fn execute(&self, ctx: SubjectContex, username: Username) -> AppResult<OwnerUserOutput> {
        Ok(self.repo.get_by_username(ctx, username.clone()).await?.into())
    }
}

