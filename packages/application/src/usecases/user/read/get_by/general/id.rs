
use domain::UserId;

use crate::{RequestContex, dto::user_dto::output::GeneralUserOutput, error::AppResult, ports::UserRepository};


pub struct GetUserByIdGenaralUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByIdGenaralUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, id: UserId) -> AppResult<GeneralUserOutput> {
            Ok(self.repo.get_by_id(ctx, id.clone()).await?.into())
    }
}
