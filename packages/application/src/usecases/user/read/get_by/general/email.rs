
use domain::Email;

use crate::{RequestContex, dto::user_dto::output::GeneralUserOutput, error::AppResult, ports::UserRepository};


pub struct GetUserByEmailGeneralUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByEmailGeneralUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, email: Email ) -> AppResult<GeneralUserOutput> {
        Ok(self.repo.get_by_email(ctx, email.clone()).await?.into())
    }
}
