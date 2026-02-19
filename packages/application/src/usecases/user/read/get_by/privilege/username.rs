use domain::Username;

use crate::{RequestContex, dto::user_dto::output::PrivilegeUserOutput, error::AppResult, ports::UserRepository};

pub struct GetUserByUsernamePrivilegeUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByUsernamePrivilegeUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, username: Username) -> AppResult<PrivilegeUserOutput> {
        Ok(self.repo.get_by_username(ctx, username.clone()).await?.into())
    }
}
