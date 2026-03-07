
use std::sync::Arc;

use domain::UserId;

use crate::{SubjectContex, authorization::ports::AuthorizationService, dto::user_dto::output::PrivilegeUserOutput, error::AppResult, ports::UserRepository};


pub struct GetUserByIdPrivilegeUseCase {
    repo: Arc<dyn UserRepository>,
    auth: Arc<dyn AuthorizationService>
}

impl GetUserByIdPrivilegeUseCase {
    pub async fn execute(&self, ctx: SubjectContex, id: UserId) -> AppResult<PrivilegeUserOutput> {
            Ok(self.repo.get_by_id(ctx, id.clone()).await?.into())
    }
}
