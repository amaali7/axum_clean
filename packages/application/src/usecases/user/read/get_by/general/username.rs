use domain::Username;

use crate::{RequestContex, dto::user_dto::output::GeneralUserOutput, error::AppResult, ports::UserRepository};

pub struct GetUserByUsernameGeneralUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByUsernameGeneralUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, username: Username) -> AppResult<GeneralUserOutput> {
        Ok(self.repo.get_by_username(ctx, username.clone()).await?.into())
    }
}
