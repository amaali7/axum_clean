
use domain::Email;

use crate::{RequestContex, dto::user_dto::output::OwnerUserOutput, error::AppResult, ports::UserRepository};


pub struct GetUserByEmailOwnerUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByEmailOwnerUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, email: Email ) -> AppResult<OwnerUserOutput> {
        Ok(self.repo.get_by_email(ctx, email.clone()).await?.into())
    }
}

