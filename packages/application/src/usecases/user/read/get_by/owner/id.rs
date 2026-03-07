use domain::UserId;

use crate::{ SubjectContex, dto::user_dto::output::OwnerUserOutput, error::AppResult, ports::UserRepository};


pub struct GetUserByIdOwnerUseCase<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> GetUserByIdOwnerUseCase<R> {
    pub async fn execute(&self, ctx: SubjectContex, id: UserId) -> AppResult<OwnerUserOutput> {
            Ok(self.repo.get_by_id(ctx, id.clone()).await?.into())
    }
}

