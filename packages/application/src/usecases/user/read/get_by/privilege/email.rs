use domain::Email;

use crate::{ RequestContex, dto::user_dto::output::PrivilegeUserOutput, error::AppResult, ports::UserRepository};

pub struct GetUserByEmailPrivilegeUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByEmailPrivilegeUseCase<R> {
    pub async fn execute(&self, ctx: RequestContex, email: Email ) -> AppResult<PrivilegeUserOutput> {
        Ok(self.repo.get_by_email(ctx, email.clone()).await?.into())
    }
}
