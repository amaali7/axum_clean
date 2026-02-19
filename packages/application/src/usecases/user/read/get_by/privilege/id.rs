
use domain::UserId;

use crate::{RequestContex, dto::user_dto::output::PrivilegeUserOutput, error::AppResult, ports::UserRepository};


pub struct GetUserByIdPrivilegeUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByIdPrivilegeUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, id: UserId) -> AppResult<PrivilegeUserOutput> {
            Ok(self.repo.get_by_id(ctx, id.clone()).await?.into())
    }
}
